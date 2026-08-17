// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery-Jacobian form scalar multiplication for P-384
// Input scalar[6], point[18]; output res[18]
//
// extern void p384_montjscalarmul_alt
//   (uint64_t res[static 18],
//    const uint64_t scalar[static 6],
//    const uint64_t point[static 18]);
//
// This function is a variant of its affine point version p384_scalarmul_alt.
// Here, input and output points are assumed to be in Jacobian form with
// their coordinates in the Montgomery domain. Thus, if priming indicates
// Montgomery form, x' = (2^384 * x) mod p_384 etc., each point argument
// is a triple (x',y',z') representing the affine point (x/z^2,y/z^3) when
// z' is nonzero or the point at infinity (group identity) if z' = 0.
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-384, returns a representation of n * P. If the result is the
// point at infinity (either because the input point was or because the
// scalar was a multiple of p_384) then the output is guaranteed to
// represent the point at infinity, i.e. to have its z coordinate zero.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------


#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjscalarmul_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjscalarmul_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjscalarmul_alt)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 48
#define JACSIZE (3*NUMSIZE)

// Safe copies of input res and additional values in variables.

#define bf x22
#define sgn x23
#define j x24
#define res x25

// Intermediate variables on the stack.
// The table is 16 entries, each of size JACSIZE = 3 * NUMSIZE

#define scalarb sp, #(0*NUMSIZE)
#define acc sp, #(1*NUMSIZE)
#define tabent sp, #(4*NUMSIZE)

#define tab sp, #(7*NUMSIZE)

#define NSPACE 55*NUMSIZE

// Avoid using .rep for the sake of the BoringSSL/AWS-LC delocator,
// which doesn't accept repetitions, assembler macros etc.

#define selectblock(I)                            \
        cmp     bf, #(1*I) __LF                      \
        ldp     x20, x21, [x19] __LF                 \
        csel    x0, x20, x0, eq __LF                 \
        csel    x1, x21, x1, eq __LF                 \
        ldp     x20, x21, [x19, #16] __LF            \
        csel    x2, x20, x2, eq __LF                 \
        csel    x3, x21, x3, eq __LF                 \
        ldp     x20, x21, [x19, #32] __LF            \
        csel    x4, x20, x4, eq __LF                 \
        csel    x5, x21, x5, eq __LF                 \
        ldp     x20, x21, [x19, #48] __LF            \
        csel    x6, x20, x6, eq __LF                 \
        csel    x7, x21, x7, eq __LF                 \
        ldp     x20, x21, [x19, #64] __LF            \
        csel    x8, x20, x8, eq __LF                 \
        csel    x9, x21, x9, eq __LF                 \
        ldp     x20, x21, [x19, #80] __LF            \
        csel    x10, x20, x10, eq __LF               \
        csel    x11, x21, x11, eq __LF               \
        ldp     x20, x21, [x19, #96] __LF            \
        csel    x12, x20, x12, eq __LF               \
        csel    x13, x21, x13, eq __LF               \
        ldp     x20, x21, [x19, #112] __LF           \
        csel    x14, x20, x14, eq __LF               \
        csel    x15, x21, x15, eq __LF               \
        ldp     x20, x21, [x19, #128] __LF           \
        csel    x16, x20, x16, eq __LF               \
        csel    x17, x21, x17, eq __LF               \
        add     x19, x19, #JACSIZE

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(p384_montjscalarmul_alt):
        CFI_START

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x30)
        CFI_DEC_SP(NSPACE)

// Preserve the "res" input argument; others get processed early.

        mov     res, x0

// Reduce the input scalar mod n_384, i.e. conditionally subtract n_384.
// Store it to "scalarb".

        ldp     x3, x4, [x1]
        movbig(x15, #0xecec, #0x196a, #0xccc5, #0x2973)
        ldp     x5, x6, [x1, #16]
        movbig(x16, #0x581a, #0x0db2, #0x48b0, #0xa77a)
        ldp     x7, x8, [x1, #32]
        movbig(x17, #0xc763, #0x4d81, #0xf437, #0x2ddf)

        subs    x9, x3, x15
        sbcs    x10, x4, x16
        sbcs    x11, x5, x17
        adcs    x12, x6, xzr
        adcs    x13, x7, xzr
        adcs    x14, x8, xzr

        csel    x3, x3, x9, cc
        csel    x4, x4, x10, cc
        csel    x5, x5, x11, cc
        csel    x6, x6, x12, cc
        csel    x7, x7, x13, cc
        csel    x8, x8, x14, cc

        stp     x3, x4, [scalarb]
        stp     x5, x6, [scalarb+16]
        stp     x7, x8, [scalarb+32]

// Set the tab[0] table entry to the input point = 1 * P

        ldp     x10, x11, [x2]
        stp     x10, x11, [tab]
        ldp     x12, x13, [x2, #16]
        stp     x12, x13, [tab+16]
        ldp     x14, x15, [x2, #32]
        stp     x14, x15, [tab+32]

        ldp     x10, x11, [x2, #48]
        stp     x10, x11, [tab+48]
        ldp     x12, x13, [x2, #64]
        stp     x12, x13, [tab+64]
        ldp     x14, x15, [x2, #80]
        stp     x14, x15, [tab+80]

        ldp     x10, x11, [x2, #96]
        stp     x10, x11, [tab+96]
        ldp     x12, x13, [x2, #112]
        stp     x12, x13, [tab+112]
        ldp     x14, x15, [x2, #128]
        stp     x14, x15, [tab+128]

// Compute and record tab[1] = 2 * p, ..., tab[15] = 16 * P

        add     x0, tab+JACSIZE*1
        add     x1, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, tab+JACSIZE*2
        add     x1, tab+JACSIZE*1
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        add     x0, tab+JACSIZE*3
        add     x1, tab+JACSIZE*1
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, tab+JACSIZE*4
        add     x1, tab+JACSIZE*3
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        add     x0, tab+JACSIZE*5
        add     x1, tab+JACSIZE*2
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, tab+JACSIZE*6
        add     x1, tab+JACSIZE*5
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        add     x0, tab+JACSIZE*7
        add     x1, tab+JACSIZE*3
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, tab+JACSIZE*8
        add     x1, tab+JACSIZE*7
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        add     x0, tab+JACSIZE*9
        add     x1, tab+JACSIZE*4
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, tab+JACSIZE*10
        add     x1, tab+JACSIZE*9
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        add     x0, tab+JACSIZE*11
        add     x1, tab+JACSIZE*5
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, tab+JACSIZE*12
        add     x1, tab+JACSIZE*11
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        add     x0, tab+JACSIZE*13
        add     x1, tab+JACSIZE*6
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, tab+JACSIZE*14
        add     x1, tab+JACSIZE*13
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        add     x0, tab+JACSIZE*15
        add     x1, tab+JACSIZE*7
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

// Add the recoding constant sum_i(16 * 32^i) to the scalar to allow signed
// digits. The digits of the constant, in lowest-to-highest order, are as
// follows; they are generated dynamically since none is a simple ARM load.
//
// 0x0842108421084210
// 0x1084210842108421
// 0x2108421084210842
// 0x4210842108421084
// 0x8421084210842108
// 0x0842108421084210

        ldp     x0, x1, [scalarb]
        ldp     x2, x3, [scalarb+16]
        ldp     x4, x5, [scalarb+32]
        movbig(x8, #0x1084, #0x2108, #0x4210, #0x8421)
        adds    x0, x0, x8, lsr #1
        adcs    x1, x1, x8
        lsl     x8, x8, #1
        adcs    x2, x2, x8
        lsl     x8, x8, #1
        adcs    x3, x3, x8
        lsl     x8, x8, #1
        adcs    x4, x4, x8
        lsr     x8, x8, #4
        adcs    x5, x5, x8
        cset    x6, cs

// Record the top bitfield then shift the whole scalar left 4 bits
// to align the top of the next bitfield with the MSB (bits 379..383).

        extr    bf, x6, x5, #60
        extr    x5, x5, x4, #60
        extr    x4, x4, x3, #60
        extr    x3, x3, x2, #60
        extr    x2, x2, x1, #60
        extr    x1, x1, x0, #60
        lsl     x0, x0, #4
        stp     x0, x1, [scalarb]
        stp     x2, x3, [scalarb+16]
        stp     x4, x5, [scalarb+32]

// Initialize the accumulator to the corresponding entry using constant-time
// lookup in the table. This top digit, uniquely, is not recoded so there is
// no sign adjustment to make.

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, xzr
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr
        mov     x12, xzr
        mov     x13, xzr
        mov     x14, xzr
        mov     x15, xzr
        mov     x16, xzr
        mov     x17, xzr

        add     x19, tab

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)
        selectblock(9)
        selectblock(10)
        selectblock(11)
        selectblock(12)
        selectblock(13)
        selectblock(14)
        selectblock(15)
        selectblock(16)

        stp     x0, x1, [acc]
        stp     x2, x3, [acc+16]
        stp     x4, x5, [acc+32]
        stp     x6, x7, [acc+48]
        stp     x8, x9, [acc+64]
        stp     x10, x11, [acc+80]
        stp     x12, x13, [acc+96]
        stp     x14, x15, [acc+112]
        stp     x16, x17, [acc+128]

        mov     j, #380

// Main loop over size-5 bitfields: double 5 times then add signed digit
// At each stage we shift the scalar left by 5 bits so we can simply pick
// the top 5 bits as the bitfield, saving some fiddle over indexing.

Lp384_montjscalarmul_alt_mainloop:
        sub     j, j, #5

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjdouble)

// Choose the bitfield and adjust it to sign and magnitude

        ldp     x0, x1, [scalarb]
        ldp     x2, x3, [scalarb+16]
        ldp     x4, x5, [scalarb+32]
        lsr     bf, x5, #59
        extr    x5, x5, x4, #59
        extr    x4, x4, x3, #59
        extr    x3, x3, x2, #59
        extr    x2, x2, x1, #59
        extr    x1, x1, x0, #59
        lsl     x0, x0, #5
        stp     x0, x1, [scalarb]
        stp     x2, x3, [scalarb+16]
        stp     x4, x5, [scalarb+32]

        subs    bf, bf, #16
        cset    sgn, lo                 // sgn = sign of digit (1 = negative)
        cneg    bf, bf, lo              // bf = absolute value of digit

// Conditionally select the table entry tab[i-1] = i * P in constant time

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, xzr
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        mov     x9, xzr
        mov     x10, xzr
        mov     x11, xzr
        mov     x12, xzr
        mov     x13, xzr
        mov     x14, xzr
        mov     x15, xzr
        mov     x16, xzr
        mov     x17, xzr

        add     x19, tab

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)
        selectblock(9)
        selectblock(10)
        selectblock(11)
        selectblock(12)
        selectblock(13)
        selectblock(14)
        selectblock(15)
        selectblock(16)

// Store it to "tabent" with the y coordinate optionally negated.
// This is done carefully to give coordinates < p_384 even in
// the degenerate case y = 0 (when z = 0 for points on the curve).

        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]
        stp     x4, x5, [tabent+32]

        stp     x12, x13, [tabent+96]
        stp     x14, x15, [tabent+112]
        stp     x16, x17, [tabent+128]

        mov     x0, #0x00000000ffffffff
        subs    x0, x0, x6
        orr     x12, x6, x7
        mov     x1, #0xffffffff00000000
        sbcs    x1, x1, x7
        orr     x13, x8, x9
        mov     x2, #0xfffffffffffffffe
        sbcs    x2, x2, x8
        orr     x14, x10, x11
        mov     x5, #0xffffffffffffffff
        sbcs    x3, x5, x9
        orr     x12, x12, x13
        sbcs    x4, x5, x10
        orr     x12, x12, x14
        sbcs    x5, x5, x11

        cmp     sgn, xzr
        ccmp    x12, xzr, #4, ne

        csel    x6, x0, x6, ne
        csel    x7, x1, x7, ne
        csel    x8, x2, x8, ne
        csel    x9, x3, x9, ne
        csel    x10, x4, x10, ne
        csel    x11, x5, x11, ne

        stp     x6, x7, [tabent+48]
        stp     x8, x9, [tabent+64]
        stp     x10, x11, [tabent+80]

// Add to the accumulator

        add     x0, acc
        add     x1, acc
        add     x2, tabent
        CFI_BL(Lp384_montjscalarmul_alt_p384_montjadd)

        cbnz    j, Lp384_montjscalarmul_alt_mainloop

// That's the end of the main loop, and we just need to copy the
// result in "acc" to the output.

        ldp     x0, x1, [acc]
        stp     x0, x1, [res]
        ldp     x0, x1, [acc+16]
        stp     x0, x1, [res, #16]
        ldp     x0, x1, [acc+32]
        stp     x0, x1, [res, #32]
        ldp     x0, x1, [acc+48]
        stp     x0, x1, [res, #48]
        ldp     x0, x1, [acc+64]
        stp     x0, x1, [res, #64]
        ldp     x0, x1, [acc+80]
        stp     x0, x1, [res, #80]
        ldp     x0, x1, [acc+96]
        stp     x0, x1, [res, #96]
        ldp     x0, x1, [acc+112]
        stp     x0, x1, [res, #112]
        ldp     x0, x1, [acc+128]
        stp     x0, x1, [res, #128]

// Restore stack and registers and return

        CFI_INC_SP(NSPACE)
        CFI_POP2(x25,x30)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p384_montjscalarmul_alt)

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjadd)

Lp384_montjscalarmul_alt_p384_montjadd:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_DEC_SP(336)
        mov     x24, x0
        mov     x25, x1
        mov     x26, x2
        ldp     x2, x3, [x25, #96]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x25, #112]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [x25, #128]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        mov     x14, #-4294967295
        mov     x15, #4294967295
        csel    x14, x14, xzr, hs
        csel    x15, x15, xzr, hs
        cset    x16, hs
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, xzr
        adcs    x12, x12, xzr
        adc     x13, x13, xzr
        stp     x2, x9, [sp]
        stp     x10, x11, [sp, #16]
        stp     x12, x13, [sp, #32]
        ldp     x2, x3, [x26, #96]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x26, #112]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [x26, #128]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        mov     x14, #-4294967295
        mov     x15, #4294967295
        csel    x14, x14, xzr, hs
        csel    x15, x15, xzr, hs
        cset    x16, hs
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, xzr
        adcs    x12, x12, xzr
        adc     x13, x13, xzr
        stp     x2, x9, [sp, #240]
        stp     x10, x11, [sp, #256]
        stp     x12, x13, [sp, #272]
        ldp     x3, x4, [x26, #96]
        ldp     x5, x6, [x25, #48]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [x25, #64]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [x25, #80]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [x26, #112]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [x26, #128]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #288]
        stp     x14, x15, [sp, #304]
        stp     x16, x17, [sp, #320]
        ldp     x3, x4, [x25, #96]
        ldp     x5, x6, [x26, #48]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [x26, #64]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [x26, #80]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [x25, #112]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [x25, #128]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #48]
        stp     x14, x15, [sp, #64]
        stp     x16, x17, [sp, #80]
        ldp     x3, x4, [sp]
        ldp     x5, x6, [x26]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [x26, #16]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [x26, #32]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #16]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #32]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #96]
        stp     x14, x15, [sp, #112]
        stp     x16, x17, [sp, #128]
        ldp     x3, x4, [sp, #240]
        ldp     x5, x6, [x25]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [x25, #16]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [x25, #32]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #256]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #272]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #192]
        stp     x14, x15, [sp, #208]
        stp     x16, x17, [sp, #224]
        ldp     x3, x4, [sp]
        ldp     x5, x6, [sp, #48]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #64]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #80]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #16]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #32]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #48]
        stp     x14, x15, [sp, #64]
        stp     x16, x17, [sp, #80]
        ldp     x3, x4, [sp, #240]
        ldp     x5, x6, [sp, #288]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #304]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #320]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #256]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #272]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #288]
        stp     x14, x15, [sp, #304]
        stp     x16, x17, [sp, #320]
        ldp     x5, x6, [sp, #96]
        ldp     x4, x3, [sp, #192]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #112]
        ldp     x4, x3, [sp, #208]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #128]
        ldp     x4, x3, [sp, #224]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #240]
        stp     x7, x8, [sp, #256]
        stp     x9, x10, [sp, #272]
        ldp     x5, x6, [sp, #48]
        ldp     x4, x3, [sp, #288]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #64]
        ldp     x4, x3, [sp, #304]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #80]
        ldp     x4, x3, [sp, #320]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #48]
        stp     x7, x8, [sp, #64]
        stp     x9, x10, [sp, #80]
        ldp     x2, x3, [sp, #240]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #256]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [sp, #272]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        mov     x14, #-4294967295
        mov     x15, #4294967295
        csel    x14, x14, xzr, hs
        csel    x15, x15, xzr, hs
        cset    x16, hs
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, xzr
        adcs    x12, x12, xzr
        adc     x13, x13, xzr
        stp     x2, x9, [sp, #144]
        stp     x10, x11, [sp, #160]
        stp     x12, x13, [sp, #176]
        ldp     x2, x3, [sp, #48]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #64]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [sp, #80]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        adc     x6, xzr, xzr
        mov     x8, #-4294967295
        adds    x14, x2, x8
        mov     x8, #4294967295
        adcs    x15, x9, x8
        mov     x8, #1
        adcs    x16, x10, x8
        adcs    x17, x11, xzr
        adcs    x19, x12, xzr
        adcs    x20, x13, xzr
        adcs    x6, x6, xzr
        csel    x2, x2, x14, eq
        csel    x9, x9, x15, eq
        csel    x10, x10, x16, eq
        csel    x11, x11, x17, eq
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        stp     x2, x9, [sp]
        stp     x10, x11, [sp, #16]
        stp     x12, x13, [sp, #32]
        ldp     x3, x4, [sp, #144]
        ldp     x5, x6, [sp, #192]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #208]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #224]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #160]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #176]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #192]
        stp     x14, x15, [sp, #208]
        stp     x16, x17, [sp, #224]
        ldp     x3, x4, [sp, #144]
        ldp     x5, x6, [sp, #96]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #112]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #128]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #160]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #176]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #96]
        stp     x14, x15, [sp, #112]
        stp     x16, x17, [sp, #128]
        ldp     x5, x6, [sp]
        ldp     x4, x3, [sp, #192]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #16]
        ldp     x4, x3, [sp, #208]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #32]
        ldp     x4, x3, [sp, #224]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp]
        stp     x7, x8, [sp, #16]
        stp     x9, x10, [sp, #32]
        ldp     x5, x6, [sp, #96]
        ldp     x4, x3, [sp, #192]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #112]
        ldp     x4, x3, [sp, #208]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #128]
        ldp     x4, x3, [sp, #224]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #144]
        stp     x7, x8, [sp, #160]
        stp     x9, x10, [sp, #176]
        ldp     x3, x4, [sp, #240]
        ldp     x5, x6, [x25, #96]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [x25, #112]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [x25, #128]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #256]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #272]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #240]
        stp     x14, x15, [sp, #256]
        stp     x16, x17, [sp, #272]
        ldp     x5, x6, [sp]
        ldp     x4, x3, [sp, #96]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #16]
        ldp     x4, x3, [sp, #112]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #32]
        ldp     x4, x3, [sp, #128]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp]
        stp     x7, x8, [sp, #16]
        stp     x9, x10, [sp, #32]
        ldp     x5, x6, [sp, #192]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #208]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #224]
        ldp     x4, x3, [sp, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #192]
        stp     x7, x8, [sp, #208]
        stp     x9, x10, [sp, #224]
        ldp     x3, x4, [sp, #144]
        ldp     x5, x6, [sp, #288]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #304]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #320]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #160]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #176]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #144]
        stp     x14, x15, [sp, #160]
        stp     x16, x17, [sp, #176]
        ldp     x3, x4, [sp, #240]
        ldp     x5, x6, [x26, #96]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [x26, #112]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [x26, #128]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #256]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #272]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #240]
        stp     x14, x15, [sp, #256]
        stp     x16, x17, [sp, #272]
        ldp     x3, x4, [sp, #48]
        ldp     x5, x6, [sp, #192]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #208]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #224]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #64]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #80]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #192]
        stp     x14, x15, [sp, #208]
        stp     x16, x17, [sp, #224]
        ldp     x5, x6, [sp, #192]
        ldp     x4, x3, [sp, #144]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #208]
        ldp     x4, x3, [sp, #160]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #224]
        ldp     x4, x3, [sp, #176]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #192]
        stp     x7, x8, [sp, #208]
        stp     x9, x10, [sp, #224]
        ldp     x0, x1, [x25, #96]
        ldp     x2, x3, [x25, #112]
        ldp     x4, x5, [x25, #128]
        orr     x20, x0, x1
        orr     x21, x2, x3
        orr     x22, x4, x5
        orr     x20, x20, x21
        orr     x20, x20, x22
        cmp     x20, xzr
        cset    x20, ne
        ldp     x6, x7, [x26, #96]
        ldp     x8, x9, [x26, #112]
        ldp     x10, x11, [x26, #128]
        orr     x21, x6, x7
        orr     x22, x8, x9
        orr     x23, x10, x11
        orr     x21, x21, x22
        orr     x21, x21, x23
        cmp     x21, xzr
        cset    x21, ne
        cmp     x21, x20
        ldp     x12, x13, [sp, #240]
        csel    x12, x0, x12, lo
        csel    x13, x1, x13, lo
        csel    x12, x6, x12, hi
        csel    x13, x7, x13, hi
        ldp     x14, x15, [sp, #256]
        csel    x14, x2, x14, lo
        csel    x15, x3, x15, lo
        csel    x14, x8, x14, hi
        csel    x15, x9, x15, hi
        ldp     x16, x17, [sp, #272]
        csel    x16, x4, x16, lo
        csel    x17, x5, x17, lo
        csel    x16, x10, x16, hi
        csel    x17, x11, x17, hi
        ldp     x20, x21, [x25]
        ldp     x0, x1, [sp]
        csel    x0, x20, x0, lo
        csel    x1, x21, x1, lo
        ldp     x20, x21, [x26]
        csel    x0, x20, x0, hi
        csel    x1, x21, x1, hi
        ldp     x20, x21, [x25, #16]
        ldp     x2, x3, [sp, #16]
        csel    x2, x20, x2, lo
        csel    x3, x21, x3, lo
        ldp     x20, x21, [x26, #16]
        csel    x2, x20, x2, hi
        csel    x3, x21, x3, hi
        ldp     x20, x21, [x25, #32]
        ldp     x4, x5, [sp, #32]
        csel    x4, x20, x4, lo
        csel    x5, x21, x5, lo
        ldp     x20, x21, [x26, #32]
        csel    x4, x20, x4, hi
        csel    x5, x21, x5, hi
        ldp     x20, x21, [x25, #48]
        ldp     x6, x7, [sp, #192]
        csel    x6, x20, x6, lo
        csel    x7, x21, x7, lo
        ldp     x20, x21, [x26, #48]
        csel    x6, x20, x6, hi
        csel    x7, x21, x7, hi
        ldp     x20, x21, [x25, #64]
        ldp     x8, x9, [sp, #208]
        csel    x8, x20, x8, lo
        csel    x9, x21, x9, lo
        ldp     x20, x21, [x26, #64]
        csel    x8, x20, x8, hi
        csel    x9, x21, x9, hi
        ldp     x20, x21, [x25, #80]
        ldp     x10, x11, [sp, #224]
        csel    x10, x20, x10, lo
        csel    x11, x21, x11, lo
        ldp     x20, x21, [x26, #80]
        csel    x10, x20, x10, hi
        csel    x11, x21, x11, hi
        stp     x0, x1, [x24]
        stp     x2, x3, [x24, #16]
        stp     x4, x5, [x24, #32]
        stp     x6, x7, [x24, #48]
        stp     x8, x9, [x24, #64]
        stp     x10, x11, [x24, #80]
        stp     x12, x13, [x24, #96]
        stp     x14, x15, [x24, #112]
        stp     x16, x17, [x24, #128]
        CFI_INC_SP(336)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjdouble)

Lp384_montjscalarmul_alt_p384_montjdouble:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_DEC_SP(336)
        mov     x23, x0
        mov     x24, x1
        ldp     x2, x3, [x24, #96]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x24, #112]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [x24, #128]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        adc     x6, xzr, xzr
        mov     x8, #-4294967295
        adds    x14, x2, x8
        mov     x8, #4294967295
        adcs    x15, x9, x8
        mov     x8, #1
        adcs    x16, x10, x8
        adcs    x17, x11, xzr
        adcs    x19, x12, xzr
        adcs    x20, x13, xzr
        adcs    x6, x6, xzr
        csel    x2, x2, x14, eq
        csel    x9, x9, x15, eq
        csel    x10, x10, x16, eq
        csel    x11, x11, x17, eq
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        stp     x2, x9, [sp]
        stp     x10, x11, [sp, #16]
        stp     x12, x13, [sp, #32]
        ldp     x2, x3, [x24, #48]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x24, #64]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [x24, #80]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        adc     x6, xzr, xzr
        mov     x8, #-4294967295
        adds    x14, x2, x8
        mov     x8, #4294967295
        adcs    x15, x9, x8
        mov     x8, #1
        adcs    x16, x10, x8
        adcs    x17, x11, xzr
        adcs    x19, x12, xzr
        adcs    x20, x13, xzr
        adcs    x6, x6, xzr
        csel    x2, x2, x14, eq
        csel    x9, x9, x15, eq
        csel    x10, x10, x16, eq
        csel    x11, x11, x17, eq
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        stp     x2, x9, [sp, #48]
        stp     x10, x11, [sp, #64]
        stp     x12, x13, [sp, #80]
        ldp     x5, x6, [x24]
        ldp     x4, x3, [sp]
        adds    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x24, #16]
        ldp     x4, x3, [sp, #16]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        ldp     x9, x10, [x24, #32]
        ldp     x4, x3, [sp, #32]
        adcs    x9, x9, x4
        adcs    x10, x10, x3
        csetm   x3, hs
        mov     x4, #4294967295
        and     x4, x4, x3
        subs    x5, x5, x4
        eor     x4, x4, x3
        sbcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        sbcs    x9, x9, x3
        sbc     x10, x10, x3
        stp     x5, x6, [sp, #240]
        stp     x7, x8, [sp, #256]
        stp     x9, x10, [sp, #272]
        ldp     x5, x6, [x24]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x24, #16]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [x24, #32]
        ldp     x4, x3, [sp, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #192]
        stp     x7, x8, [sp, #208]
        stp     x9, x10, [sp, #224]
        ldp     x3, x4, [sp, #240]
        ldp     x5, x6, [sp, #192]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #208]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #224]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #256]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #272]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #96]
        stp     x14, x15, [sp, #112]
        stp     x16, x17, [sp, #128]
        ldp     x5, x6, [x24, #48]
        ldp     x4, x3, [x24, #96]
        adds    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x24, #64]
        ldp     x4, x3, [x24, #112]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        ldp     x9, x10, [x24, #80]
        ldp     x4, x3, [x24, #128]
        adcs    x9, x9, x4
        adcs    x10, x10, x3
        adc     x3, xzr, xzr
        mov     x4, #4294967295
        cmp     x5, x4
        mov     x4, #-4294967296
        sbcs    xzr, x6, x4
        mov     x4, #-2
        sbcs    xzr, x7, x4
        adcs    xzr, x8, xzr
        adcs    xzr, x9, xzr
        adcs    xzr, x10, xzr
        adcs    x3, x3, xzr
        csetm   x3, ne
        mov     x4, #4294967295
        and     x4, x4, x3
        subs    x5, x5, x4
        eor     x4, x4, x3
        sbcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        sbcs    x9, x9, x3
        sbc     x10, x10, x3
        stp     x5, x6, [sp, #240]
        stp     x7, x8, [sp, #256]
        stp     x9, x10, [sp, #272]
        ldp     x2, x3, [sp, #96]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #112]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [sp, #128]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        adc     x6, xzr, xzr
        mov     x8, #-4294967295
        adds    x14, x2, x8
        mov     x8, #4294967295
        adcs    x15, x9, x8
        mov     x8, #1
        adcs    x16, x10, x8
        adcs    x17, x11, xzr
        adcs    x19, x12, xzr
        adcs    x20, x13, xzr
        adcs    x6, x6, xzr
        csel    x2, x2, x14, eq
        csel    x9, x9, x15, eq
        csel    x10, x10, x16, eq
        csel    x11, x11, x17, eq
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        stp     x2, x9, [sp, #288]
        stp     x10, x11, [sp, #304]
        stp     x12, x13, [sp, #320]
        ldp     x3, x4, [x24]
        ldp     x5, x6, [sp, #48]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #64]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #80]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [x24, #16]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [x24, #32]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #144]
        stp     x14, x15, [sp, #160]
        stp     x16, x17, [sp, #176]
        ldp     x2, x3, [sp, #240]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #256]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [sp, #272]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        adc     x6, xzr, xzr
        mov     x8, #-4294967295
        adds    x14, x2, x8
        mov     x8, #4294967295
        adcs    x15, x9, x8
        mov     x8, #1
        adcs    x16, x10, x8
        adcs    x17, x11, xzr
        adcs    x19, x12, xzr
        adcs    x20, x13, xzr
        adcs    x6, x6, xzr
        csel    x2, x2, x14, eq
        csel    x9, x9, x15, eq
        csel    x10, x10, x16, eq
        csel    x11, x11, x17, eq
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        stp     x2, x9, [sp, #192]
        stp     x10, x11, [sp, #208]
        stp     x12, x13, [sp, #224]
        ldp     x0, x1, [sp, #288]
        mov     x6, #4294967295
        subs    x6, x6, x0
        mov     x7, #-4294967296
        sbcs    x7, x7, x1
        ldp     x0, x1, [sp, #304]
        mov     x8, #-2
        sbcs    x8, x8, x0
        mov     x13, #-1
        sbcs    x9, x13, x1
        ldp     x0, x1, [sp, #320]
        sbcs    x10, x13, x0
        sbc     x11, x13, x1
        mov     x12, #9
        mul     x0, x12, x6
        mul     x1, x12, x7
        mul     x2, x12, x8
        mul     x3, x12, x9
        mul     x4, x12, x10
        mul     x5, x12, x11
        umulh   x6, x12, x6
        umulh   x7, x12, x7
        umulh   x8, x12, x8
        umulh   x9, x12, x9
        umulh   x10, x12, x10
        umulh   x12, x12, x11
        adds    x1, x1, x6
        adcs    x2, x2, x7
        adcs    x3, x3, x8
        adcs    x4, x4, x9
        adcs    x5, x5, x10
        mov     x6, #1
        adc     x6, x12, x6
        ldp     x8, x9, [sp, #144]
        ldp     x10, x11, [sp, #160]
        ldp     x12, x13, [sp, #176]
        mov     x14, #12
        mul     x15, x14, x8
        umulh   x8, x14, x8
        adds    x0, x0, x15
        mul     x15, x14, x9
        umulh   x9, x14, x9
        adcs    x1, x1, x15
        mul     x15, x14, x10
        umulh   x10, x14, x10
        adcs    x2, x2, x15
        mul     x15, x14, x11
        umulh   x11, x14, x11
        adcs    x3, x3, x15
        mul     x15, x14, x12
        umulh   x12, x14, x12
        adcs    x4, x4, x15
        mul     x15, x14, x13
        umulh   x13, x14, x13
        adcs    x5, x5, x15
        adc     x6, x6, xzr
        adds    x1, x1, x8
        adcs    x2, x2, x9
        adcs    x3, x3, x10
        adcs    x4, x4, x11
        adcs    x5, x5, x12
        adcs    x6, x6, x13
        lsl     x7, x6, #32
        subs    x8, x6, x7
        sbc     x7, x7, xzr
        adds    x0, x0, x8
        adcs    x1, x1, x7
        adcs    x2, x2, x6
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        adcs    x5, x5, xzr
        csetm   x6, lo
        mov     x7, #4294967295
        and     x7, x7, x6
        adds    x0, x0, x7
        eor     x7, x7, x6
        adcs    x1, x1, x7
        mov     x7, #-2
        and     x7, x7, x6
        adcs    x2, x2, x7
        adcs    x3, x3, x6
        adcs    x4, x4, x6
        adc     x5, x5, x6
        stp     x0, x1, [sp, #288]
        stp     x2, x3, [sp, #304]
        stp     x4, x5, [sp, #320]
        ldp     x5, x6, [sp, #192]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #208]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #224]
        ldp     x4, x3, [sp, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #240]
        stp     x7, x8, [sp, #256]
        stp     x9, x10, [sp, #272]
        ldp     x2, x3, [sp, #48]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #64]
        mul     x8, x2, x4
        adds    x10, x10, x8
        mul     x11, x2, x5
        mul     x8, x3, x4
        adcs    x11, x11, x8
        umulh   x12, x2, x5
        mul     x8, x3, x5
        adcs    x12, x12, x8
        ldp     x6, x7, [sp, #80]
        mul     x13, x2, x7
        mul     x8, x3, x6
        adcs    x13, x13, x8
        umulh   x14, x2, x7
        mul     x8, x3, x7
        adcs    x14, x14, x8
        mul     x15, x5, x6
        adcs    x15, x15, xzr
        umulh   x16, x5, x6
        adc     x16, x16, xzr
        umulh   x8, x2, x4
        adds    x11, x11, x8
        umulh   x8, x3, x4
        adcs    x12, x12, x8
        umulh   x8, x3, x5
        adcs    x13, x13, x8
        umulh   x8, x3, x6
        adcs    x14, x14, x8
        umulh   x8, x3, x7
        adcs    x15, x15, x8
        adc     x16, x16, xzr
        mul     x8, x2, x6
        adds    x12, x12, x8
        mul     x8, x4, x5
        adcs    x13, x13, x8
        mul     x8, x4, x6
        adcs    x14, x14, x8
        mul     x8, x4, x7
        adcs    x15, x15, x8
        mul     x8, x5, x7
        adcs    x16, x16, x8
        mul     x17, x6, x7
        adcs    x17, x17, xzr
        umulh   x19, x6, x7
        adc     x19, x19, xzr
        umulh   x8, x2, x6
        adds    x13, x13, x8
        umulh   x8, x4, x5
        adcs    x14, x14, x8
        umulh   x8, x4, x6
        adcs    x15, x15, x8
        umulh   x8, x4, x7
        adcs    x16, x16, x8
        umulh   x8, x5, x7
        adcs    x17, x17, x8
        adc     x19, x19, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        cset    x20, hs
        umulh   x8, x2, x2
        mul     x2, x2, x2
        adds    x9, x9, x8
        mul     x8, x3, x3
        adcs    x10, x10, x8
        umulh   x8, x3, x3
        adcs    x11, x11, x8
        mul     x8, x4, x4
        adcs    x12, x12, x8
        umulh   x8, x4, x4
        adcs    x13, x13, x8
        mul     x8, x5, x5
        adcs    x14, x14, x8
        umulh   x8, x5, x5
        adcs    x15, x15, x8
        mul     x8, x6, x6
        adcs    x16, x16, x8
        umulh   x8, x6, x6
        adcs    x17, x17, x8
        mul     x8, x7, x7
        adcs    x19, x19, x8
        umulh   x8, x7, x7
        adc     x20, x20, x8
        lsl     x5, x2, #32
        add     x2, x5, x2
        mov     x5, #-4294967295
        umulh   x5, x5, x2
        mov     x4, #4294967295
        mul     x3, x4, x2
        umulh   x4, x4, x2
        adds    x5, x5, x3
        adcs    x4, x4, x2
        adc     x3, xzr, xzr
        subs    x9, x9, x5
        sbcs    x10, x10, x4
        sbcs    x11, x11, x3
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x2, x2, xzr
        lsl     x5, x9, #32
        add     x9, x5, x9
        mov     x5, #-4294967295
        umulh   x5, x5, x9
        mov     x4, #4294967295
        mul     x3, x4, x9
        umulh   x4, x4, x9
        adds    x5, x5, x3
        adcs    x4, x4, x9
        adc     x3, xzr, xzr
        subs    x10, x10, x5
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        sbcs    x13, x13, xzr
        sbcs    x2, x2, xzr
        sbc     x9, x9, xzr
        lsl     x5, x10, #32
        add     x10, x5, x10
        mov     x5, #-4294967295
        umulh   x5, x5, x10
        mov     x4, #4294967295
        mul     x3, x4, x10
        umulh   x4, x4, x10
        adds    x5, x5, x3
        adcs    x4, x4, x10
        adc     x3, xzr, xzr
        subs    x11, x11, x5
        sbcs    x12, x12, x4
        sbcs    x13, x13, x3
        sbcs    x2, x2, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        lsl     x5, x11, #32
        add     x11, x5, x11
        mov     x5, #-4294967295
        umulh   x5, x5, x11
        mov     x4, #4294967295
        mul     x3, x4, x11
        umulh   x4, x4, x11
        adds    x5, x5, x3
        adcs    x4, x4, x11
        adc     x3, xzr, xzr
        subs    x12, x12, x5
        sbcs    x13, x13, x4
        sbcs    x2, x2, x3
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbc     x11, x11, xzr
        lsl     x5, x12, #32
        add     x12, x5, x12
        mov     x5, #-4294967295
        umulh   x5, x5, x12
        mov     x4, #4294967295
        mul     x3, x4, x12
        umulh   x4, x4, x12
        adds    x5, x5, x3
        adcs    x4, x4, x12
        adc     x3, xzr, xzr
        subs    x13, x13, x5
        sbcs    x2, x2, x4
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbc     x12, x12, xzr
        lsl     x5, x13, #32
        add     x13, x5, x13
        mov     x5, #-4294967295
        umulh   x5, x5, x13
        mov     x4, #4294967295
        mul     x3, x4, x13
        umulh   x4, x4, x13
        adds    x5, x5, x3
        adcs    x4, x4, x13
        adc     x3, xzr, xzr
        subs    x2, x2, x5
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        adds    x2, x2, x14
        adcs    x9, x9, x15
        adcs    x10, x10, x16
        adcs    x11, x11, x17
        adcs    x12, x12, x19
        adcs    x13, x13, x20
        adc     x6, xzr, xzr
        mov     x8, #-4294967295
        adds    x14, x2, x8
        mov     x8, #4294967295
        adcs    x15, x9, x8
        mov     x8, #1
        adcs    x16, x10, x8
        adcs    x17, x11, xzr
        adcs    x19, x12, xzr
        adcs    x20, x13, xzr
        adcs    x6, x6, xzr
        csel    x2, x2, x14, eq
        csel    x9, x9, x15, eq
        csel    x10, x10, x16, eq
        csel    x11, x11, x17, eq
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        stp     x2, x9, [sp, #192]
        stp     x10, x11, [sp, #208]
        stp     x12, x13, [sp, #224]
        ldp     x5, x6, [sp, #240]
        ldp     x4, x3, [sp, #48]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #256]
        ldp     x4, x3, [sp, #64]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #272]
        ldp     x4, x3, [sp, #80]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, lo
        mov     x4, #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [x23, #96]
        stp     x7, x8, [x23, #112]
        stp     x9, x10, [x23, #128]
        ldp     x3, x4, [sp, #288]
        ldp     x5, x6, [sp, #96]
        mul     x12, x3, x5
        umulh   x13, x3, x5
        mul     x11, x3, x6
        umulh   x14, x3, x6
        adds    x13, x13, x11
        ldp     x7, x8, [sp, #112]
        mul     x11, x3, x7
        umulh   x15, x3, x7
        adcs    x14, x14, x11
        mul     x11, x3, x8
        umulh   x16, x3, x8
        adcs    x15, x15, x11
        ldp     x9, x10, [sp, #128]
        mul     x11, x3, x9
        umulh   x17, x3, x9
        adcs    x16, x16, x11
        mul     x11, x3, x10
        umulh   x19, x3, x10
        adcs    x17, x17, x11
        adc     x19, x19, xzr
        mul     x11, x4, x5
        adds    x13, x13, x11
        mul     x11, x4, x6
        adcs    x14, x14, x11
        mul     x11, x4, x7
        adcs    x15, x15, x11
        mul     x11, x4, x8
        adcs    x16, x16, x11
        mul     x11, x4, x9
        adcs    x17, x17, x11
        mul     x11, x4, x10
        adcs    x19, x19, x11
        cset    x20, hs
        umulh   x11, x4, x5
        adds    x14, x14, x11
        umulh   x11, x4, x6
        adcs    x15, x15, x11
        umulh   x11, x4, x7
        adcs    x16, x16, x11
        umulh   x11, x4, x8
        adcs    x17, x17, x11
        umulh   x11, x4, x9
        adcs    x19, x19, x11
        umulh   x11, x4, x10
        adc     x20, x20, x11
        ldp     x3, x4, [sp, #304]
        mul     x11, x3, x5
        adds    x14, x14, x11
        mul     x11, x3, x6
        adcs    x15, x15, x11
        mul     x11, x3, x7
        adcs    x16, x16, x11
        mul     x11, x3, x8
        adcs    x17, x17, x11
        mul     x11, x3, x9
        adcs    x19, x19, x11
        mul     x11, x3, x10
        adcs    x20, x20, x11
        cset    x21, hs
        umulh   x11, x3, x5
        adds    x15, x15, x11
        umulh   x11, x3, x6
        adcs    x16, x16, x11
        umulh   x11, x3, x7
        adcs    x17, x17, x11
        umulh   x11, x3, x8
        adcs    x19, x19, x11
        umulh   x11, x3, x9
        adcs    x20, x20, x11
        umulh   x11, x3, x10
        adc     x21, x21, x11
        mul     x11, x4, x5
        adds    x15, x15, x11
        mul     x11, x4, x6
        adcs    x16, x16, x11
        mul     x11, x4, x7
        adcs    x17, x17, x11
        mul     x11, x4, x8
        adcs    x19, x19, x11
        mul     x11, x4, x9
        adcs    x20, x20, x11
        mul     x11, x4, x10
        adcs    x21, x21, x11
        cset    x22, hs
        umulh   x11, x4, x5
        adds    x16, x16, x11
        umulh   x11, x4, x6
        adcs    x17, x17, x11
        umulh   x11, x4, x7
        adcs    x19, x19, x11
        umulh   x11, x4, x8
        adcs    x20, x20, x11
        umulh   x11, x4, x9
        adcs    x21, x21, x11
        umulh   x11, x4, x10
        adc     x22, x22, x11
        ldp     x3, x4, [sp, #320]
        mul     x11, x3, x5
        adds    x16, x16, x11
        mul     x11, x3, x6
        adcs    x17, x17, x11
        mul     x11, x3, x7
        adcs    x19, x19, x11
        mul     x11, x3, x8
        adcs    x20, x20, x11
        mul     x11, x3, x9
        adcs    x21, x21, x11
        mul     x11, x3, x10
        adcs    x22, x22, x11
        cset    x2, hs
        umulh   x11, x3, x5
        adds    x17, x17, x11
        umulh   x11, x3, x6
        adcs    x19, x19, x11
        umulh   x11, x3, x7
        adcs    x20, x20, x11
        umulh   x11, x3, x8
        adcs    x21, x21, x11
        umulh   x11, x3, x9
        adcs    x22, x22, x11
        umulh   x11, x3, x10
        adc     x2, x2, x11
        mul     x11, x4, x5
        adds    x17, x17, x11
        mul     x11, x4, x6
        adcs    x19, x19, x11
        mul     x11, x4, x7
        adcs    x20, x20, x11
        mul     x11, x4, x8
        adcs    x21, x21, x11
        mul     x11, x4, x9
        adcs    x22, x22, x11
        mul     x11, x4, x10
        adcs    x2, x2, x11
        cset    x1, hs
        umulh   x11, x4, x5
        adds    x19, x19, x11
        umulh   x11, x4, x6
        adcs    x20, x20, x11
        umulh   x11, x4, x7
        adcs    x21, x21, x11
        umulh   x11, x4, x8
        adcs    x22, x22, x11
        umulh   x11, x4, x9
        adcs    x2, x2, x11
        umulh   x11, x4, x10
        adc     x1, x1, x11
        lsl     x7, x12, #32
        add     x12, x7, x12
        mov     x7, #-4294967295
        umulh   x7, x7, x12
        mov     x6, #4294967295
        mul     x5, x6, x12
        umulh   x6, x6, x12
        adds    x7, x7, x5
        adcs    x6, x6, x12
        adc     x5, xzr, xzr
        subs    x13, x13, x7
        sbcs    x14, x14, x6
        sbcs    x15, x15, x5
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x12, x12, xzr
        lsl     x7, x13, #32
        add     x13, x7, x13
        mov     x7, #-4294967295
        umulh   x7, x7, x13
        mov     x6, #4294967295
        mul     x5, x6, x13
        umulh   x6, x6, x13
        adds    x7, x7, x5
        adcs    x6, x6, x13
        adc     x5, xzr, xzr
        subs    x14, x14, x7
        sbcs    x15, x15, x6
        sbcs    x16, x16, x5
        sbcs    x17, x17, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        lsl     x7, x14, #32
        add     x14, x7, x14
        mov     x7, #-4294967295
        umulh   x7, x7, x14
        mov     x6, #4294967295
        mul     x5, x6, x14
        umulh   x6, x6, x14
        adds    x7, x7, x5
        adcs    x6, x6, x14
        adc     x5, xzr, xzr
        subs    x15, x15, x7
        sbcs    x16, x16, x6
        sbcs    x17, x17, x5
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        lsl     x7, x15, #32
        add     x15, x7, x15
        mov     x7, #-4294967295
        umulh   x7, x7, x15
        mov     x6, #4294967295
        mul     x5, x6, x15
        umulh   x6, x6, x15
        adds    x7, x7, x5
        adcs    x6, x6, x15
        adc     x5, xzr, xzr
        subs    x16, x16, x7
        sbcs    x17, x17, x6
        sbcs    x12, x12, x5
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        lsl     x7, x16, #32
        add     x16, x7, x16
        mov     x7, #-4294967295
        umulh   x7, x7, x16
        mov     x6, #4294967295
        mul     x5, x6, x16
        umulh   x6, x6, x16
        adds    x7, x7, x5
        adcs    x6, x6, x16
        adc     x5, xzr, xzr
        subs    x17, x17, x7
        sbcs    x12, x12, x6
        sbcs    x13, x13, x5
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbc     x16, x16, xzr
        lsl     x7, x17, #32
        add     x17, x7, x17
        mov     x7, #-4294967295
        umulh   x7, x7, x17
        mov     x6, #4294967295
        mul     x5, x6, x17
        umulh   x6, x6, x17
        adds    x7, x7, x5
        adcs    x6, x6, x17
        adc     x5, xzr, xzr
        subs    x12, x12, x7
        sbcs    x13, x13, x6
        sbcs    x14, x14, x5
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbc     x17, x17, xzr
        adds    x12, x12, x19
        adcs    x13, x13, x20
        adcs    x14, x14, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x2
        adcs    x17, x17, x1
        adc     x10, xzr, xzr
        mov     x11, #-4294967295
        adds    x19, x12, x11
        mov     x11, #4294967295
        adcs    x20, x13, x11
        mov     x11, #1
        adcs    x21, x14, x11
        adcs    x22, x15, xzr
        adcs    x2, x16, xzr
        adcs    x1, x17, xzr
        adcs    x10, x10, xzr
        csel    x12, x12, x19, eq
        csel    x13, x13, x20, eq
        csel    x14, x14, x21, eq
        csel    x15, x15, x22, eq
        csel    x16, x16, x2, eq
        csel    x17, x17, x1, eq
        stp     x12, x13, [sp, #240]
        stp     x14, x15, [sp, #256]
        stp     x16, x17, [sp, #272]
        ldp     x1, x2, [sp, #144]
        ldp     x3, x4, [sp, #160]
        ldp     x5, x6, [sp, #176]
        lsl     x0, x1, #2
        ldp     x7, x8, [sp, #288]
        subs    x0, x0, x7
        extr    x1, x2, x1, #62
        sbcs    x1, x1, x8
        ldp     x7, x8, [sp, #304]
        extr    x2, x3, x2, #62
        sbcs    x2, x2, x7
        extr    x3, x4, x3, #62
        sbcs    x3, x3, x8
        extr    x4, x5, x4, #62
        ldp     x7, x8, [sp, #320]
        sbcs    x4, x4, x7
        extr    x5, x6, x5, #62
        sbcs    x5, x5, x8
        lsr     x6, x6, #62
        adc     x6, x6, xzr
        lsl     x7, x6, #32
        subs    x8, x6, x7
        sbc     x7, x7, xzr
        adds    x0, x0, x8
        adcs    x1, x1, x7
        adcs    x2, x2, x6
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        adcs    x5, x5, xzr
        csetm   x8, lo
        mov     x9, #4294967295
        and     x9, x9, x8
        adds    x0, x0, x9
        eor     x9, x9, x8
        adcs    x1, x1, x9
        mov     x9, #-2
        and     x9, x9, x8
        adcs    x2, x2, x9
        adcs    x3, x3, x8
        adcs    x4, x4, x8
        adc     x5, x5, x8
        stp     x0, x1, [x23]
        stp     x2, x3, [x23, #16]
        stp     x4, x5, [x23, #32]
        ldp     x0, x1, [sp, #192]
        mov     x6, #4294967295
        subs    x6, x6, x0
        mov     x7, #-4294967296
        sbcs    x7, x7, x1
        ldp     x0, x1, [sp, #208]
        mov     x8, #-2
        sbcs    x8, x8, x0
        mov     x13, #-1
        sbcs    x9, x13, x1
        ldp     x0, x1, [sp, #224]
        sbcs    x10, x13, x0
        sbc     x11, x13, x1
        lsl     x0, x6, #3
        extr    x1, x7, x6, #61
        extr    x2, x8, x7, #61
        extr    x3, x9, x8, #61
        extr    x4, x10, x9, #61
        extr    x5, x11, x10, #61
        lsr     x6, x11, #61
        add     x6, x6, #1
        ldp     x8, x9, [sp, #240]
        ldp     x10, x11, [sp, #256]
        ldp     x12, x13, [sp, #272]
        mov     x14, #3
        mul     x15, x14, x8
        umulh   x8, x14, x8
        adds    x0, x0, x15
        mul     x15, x14, x9
        umulh   x9, x14, x9
        adcs    x1, x1, x15
        mul     x15, x14, x10
        umulh   x10, x14, x10
        adcs    x2, x2, x15
        mul     x15, x14, x11
        umulh   x11, x14, x11
        adcs    x3, x3, x15
        mul     x15, x14, x12
        umulh   x12, x14, x12
        adcs    x4, x4, x15
        mul     x15, x14, x13
        umulh   x13, x14, x13
        adcs    x5, x5, x15
        adc     x6, x6, xzr
        adds    x1, x1, x8
        adcs    x2, x2, x9
        adcs    x3, x3, x10
        adcs    x4, x4, x11
        adcs    x5, x5, x12
        adcs    x6, x6, x13
        lsl     x7, x6, #32
        subs    x8, x6, x7
        sbc     x7, x7, xzr
        adds    x0, x0, x8
        adcs    x1, x1, x7
        adcs    x2, x2, x6
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        adcs    x5, x5, xzr
        csetm   x6, lo
        mov     x7, #4294967295
        and     x7, x7, x6
        adds    x0, x0, x7
        eor     x7, x7, x6
        adcs    x1, x1, x7
        mov     x7, #-2
        and     x7, x7, x6
        adcs    x2, x2, x7
        adcs    x3, x3, x6
        adcs    x4, x4, x6
        adc     x5, x5, x6
        stp     x0, x1, [x23, #48]
        stp     x2, x3, [x23, #64]
        stp     x4, x5, [x23, #80]
        CFI_INC_SP(336)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
