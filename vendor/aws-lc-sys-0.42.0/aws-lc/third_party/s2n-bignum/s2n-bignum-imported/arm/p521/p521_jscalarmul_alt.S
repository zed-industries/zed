// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Jacobian form scalar multiplication for P-521
// Input scalar[9], point[27]; output res[27]
//
// extern void p521_jscalarmul_alt
//   (uint64_t res[static 27],
//    const uint64_t scalar[static 9],
//    const uint64_t point[static 27]);
//
// This function is a variant of its affine point version p521_scalarmul.
// Here, input and output points are assumed to be in Jacobian form with
// a triple (x,y,z) representing the affine point (x/z^2,y/z^3) when
// z is nonzero or the point at infinity (group identity) if z = 0.
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-521, returns a representation of n * P. If the result is the
// point at infinity (either because the input point was or because the
// scalar was a multiple of p_521) then the output is guaranteed to
// represent the point at infinity, i.e. to have its z coordinate zero.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------


#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jscalarmul_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jscalarmul_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jscalarmul_alt)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 72
#define JACSIZE (3*NUMSIZE)

// Safe copies of input res and additional values in variables.

#define tabup x15
#define bf x16
#define sgn x17
#define j x19
#define res x20

// Intermediate variables on the stack.
// The table is 16 entries, each of size JACSIZE = 3 * NUMSIZE

#define scalarb sp, #(0*NUMSIZE)
#define acc sp, #(1*NUMSIZE)
#define tabent sp, #(4*NUMSIZE)

#define tab sp, #(7*NUMSIZE)

// Round up to maintain stack alignment

#define NSPACE 3968

#define selectblock(I)                            \
        cmp     bf, #(1*I) __LF                      \
        ldp     x10, x11, [tabup] __LF               \
        csel    x0, x10, x0, eq __LF                 \
        csel    x1, x11, x1, eq __LF                 \
        ldp     x10, x11, [tabup, #16] __LF          \
        csel    x2, x10, x2, eq __LF                 \
        csel    x3, x11, x3, eq __LF                 \
        ldp     x10, x11, [tabup, #32] __LF          \
        csel    x4, x10, x4, eq __LF                 \
        csel    x5, x11, x5, eq __LF                 \
        ldp     x10, x11, [tabup, #48] __LF          \
        csel    x6, x10, x6, eq __LF                 \
        csel    x7, x11, x7, eq __LF                 \
        ldr     x10, [tabup, #64] __LF               \
        csel    x8, x10, x8, eq __LF                 \
        add     tabup, tabup, #JACSIZE

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(p521_jscalarmul_alt):
        CFI_START

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x30)
        CFI_DEC_SP(NSPACE)

// Preserve the "res" input argument; others get processed early.

        mov     res, x0

// Reduce the input scalar mod n_521 and store it to "scalarb".

        mov     x19, x2
        add     x0, scalarb
        CFI_BL(Lp521_jscalarmul_alt_bignum_mod_n521_9)
        mov     x2, x19

// Set the tab[0] table entry to the input point = 1 * P, but also
// reduce all coordinates modulo p. In principle we assume reduction
// as a precondition, but this reduces the scope for surprise, e.g.
// making sure that any input with z = 0 is treated as zero, even
// if the other coordinates are not in fact reduced.

        add     x0, tab
        mov     x1, x19
        CFI_BL(Lp521_jscalarmul_alt_bignum_mod_p521_9)

        add     x0, tab+NUMSIZE
        add     x1, x19, #NUMSIZE
        CFI_BL(Lp521_jscalarmul_alt_bignum_mod_p521_9)

        add     x0, tab+2*NUMSIZE
        add     x1, x19, #(2*NUMSIZE)
        CFI_BL(Lp521_jscalarmul_alt_bignum_mod_p521_9)

// If bit 520 of the scalar is set, then negate the scalar mod n_521,
// i.e. do scalar |-> n_521 - scalar, and also the point to compensate
// by negating its y coordinate. This further step is not needed by
// the indexing scheme (the top window is only a couple of bits either
// way), but is convenient to exclude a problem with the specific value
// scalar = n_521 - 18, where the last Jacobian addition is of the form
// (n_521 - 9) * P + -(9 * P) and hence is a degenerate doubling case.

        ldp     x0, x1, [scalarb]
        movbig(x10, #0xbb6f, #0xb71e, #0x9138, #0x6409)
        subs    x10, x10, x0
        movbig(x11, #0x3bb5, #0xc9b8, #0x899c, #0x47ae)
        sbcs    x11, x11, x1
        ldp     x2, x3, [scalarb+16]
        movbig(x12, #0x7fcc, #0x0148, #0xf709, #0xa5d0)
        sbcs    x12, x12, x2
        movbig(x13, #0x5186, #0x8783, #0xbf2f, #0x966b)
        sbcs    x13, x13, x3
        ldp     x4, x5, [scalarb+32]
        mov     x14, 0xfffffffffffffffa
        sbcs    x14, x14, x4
        mov     x15, 0xffffffffffffffff
        sbcs    x15, x15, x5
        ldp     x6, x7, [scalarb+48]
        mov     x16, 0xffffffffffffffff
        sbcs    x16, x16, x6
        mov     x17, 0xffffffffffffffff
        sbcs    x17, x17, x7
        ldr     x8, [scalarb+64]
        mov     x19, 0x00000000000001ff
        sbc     x19, x19, x8
        tst     x8, 0x100
        csetm   x9, ne
        csel    x0, x10, x0, ne
        csel    x1, x11, x1, ne
        csel    x2, x12, x2, ne
        csel    x3, x13, x3, ne
        csel    x4, x14, x4, ne
        csel    x5, x15, x5, ne
        csel    x6, x16, x6, ne
        csel    x7, x17, x7, ne
        csel    x8, x19, x8, ne
        stp     x0, x1, [scalarb]
        stp     x2, x3, [scalarb+16]
        stp     x4, x5, [scalarb+32]
        stp     x6, x7, [scalarb+48]
        str     x8, [scalarb+64]

        add     tabup, tab
        ldp     x0, x1, [tabup, #NUMSIZE]
        ldp     x2, x3, [tabup, #NUMSIZE+16]
        ldp     x4, x5, [tabup, #NUMSIZE+32]
        ldp     x6, x7, [tabup, #NUMSIZE+48]
        ldr     x8, [tabup, #NUMSIZE+64]
        orr     x10, x0, x1
        orr     x11, x2, x3
        orr     x12, x4, x5
        orr     x13, x6, x7
        orr     x10, x10, x11
        orr     x12, x12, x13
        orr     x12, x12, x8
        orr     x10, x10, x12
        cmp     x10, xzr
        csel    x9, x9, xzr, ne
        eor     x0, x0, x9
        eor     x1, x1, x9
        eor     x2, x2, x9
        eor     x3, x3, x9
        eor     x4, x4, x9
        eor     x5, x5, x9
        eor     x6, x6, x9
        eor     x7, x7, x9
        and     x9, x9, #0x1FF
        eor     x8, x8, x9
        stp     x0, x1, [tabup, #NUMSIZE]
        stp     x2, x3, [tabup, #NUMSIZE+16]
        stp     x4, x5, [tabup, #NUMSIZE+32]
        stp     x6, x7, [tabup, #NUMSIZE+48]
        str     x8, [tabup, #NUMSIZE+64]

// Compute and record tab[1] = 2 * p, ..., tab[15] = 16 * P

        add     x0, tab+JACSIZE*1
        add     x1, tab
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, tab+JACSIZE*2
        add     x1, tab+JACSIZE*1
        add     x2, tab
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        add     x0, tab+JACSIZE*3
        add     x1, tab+JACSIZE*1
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, tab+JACSIZE*4
        add     x1, tab+JACSIZE*3
        add     x2, tab
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        add     x0, tab+JACSIZE*5
        add     x1, tab+JACSIZE*2
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, tab+JACSIZE*6
        add     x1, tab+JACSIZE*5
        add     x2, tab
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        add     x0, tab+JACSIZE*7
        add     x1, tab+JACSIZE*3
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, tab+JACSIZE*8
        add     x1, tab+JACSIZE*7
        add     x2, tab
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        add     x0, tab+JACSIZE*9
        add     x1, tab+JACSIZE*4
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, tab+JACSIZE*10
        add     x1, tab+JACSIZE*9
        add     x2, tab
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        add     x0, tab+JACSIZE*11
        add     x1, tab+JACSIZE*5
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, tab+JACSIZE*12
        add     x1, tab+JACSIZE*11
        add     x2, tab
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        add     x0, tab+JACSIZE*13
        add     x1, tab+JACSIZE*6
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, tab+JACSIZE*14
        add     x1, tab+JACSIZE*13
        add     x2, tab
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        add     x0, tab+JACSIZE*15
        add     x1, tab+JACSIZE*7
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

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
// 0x1084210842108421
// 0x2108421084210842
// 0x0000000000000084

        ldp     x0, x1, [scalarb]
        ldp     x2, x3, [scalarb+16]
        ldp     x4, x5, [scalarb+32]
        ldp     x6, x7, [scalarb+48]
        ldr     x8, [scalarb+64]

        movbig(x10, #0x1084, #0x2108, #0x4210, #0x8421)
        adds    x0, x0, x10, lsr #1
        adcs    x1, x1, x10
        lsl     x10, x10, #1
        adcs    x2, x2, x10
        lsl     x10, x10, #1
        adcs    x3, x3, x10
        lsl     x10, x10, #1
        adcs    x4, x4, x10
        lsr     x11, x10, #4
        adcs    x5, x5, x11
        lsr     x10, x10, #3
        adcs    x6, x6, x10
        lsl     x10, x10, #1
        adcs    x7, x7, x10
        lsl     x10, x10, #1
        and     x10, x10, #0xFF
        adc     x8, x8, x10

// Because of the initial reduction the top bitfield (>= bits 520) is <= 1,
// i.e. just a single bit. Record that in "bf", then shift the whole
// scalar left 56 bits to align the top of the next bitfield with the MSB
// (bits 571..575).

        lsr     bf, x8, #8
        extr    x8, x8, x7, #8
        extr    x7, x7, x6, #8
        extr    x6, x6, x5, #8
        extr    x5, x5, x4, #8
        extr    x4, x4, x3, #8
        extr    x3, x3, x2, #8
        extr    x2, x2, x1, #8
        extr    x1, x1, x0, #8
        lsl     x0, x0, #56
        stp     x0, x1, [scalarb]
        stp     x2, x3, [scalarb+16]
        stp     x4, x5, [scalarb+32]
        stp     x6, x7, [scalarb+48]
        str     x8, [scalarb+64]

// According to the top bit, initialize the accumulator to P or 0. This top
// digit, uniquely, is not recoded so there is no sign adjustment to make.
// We only really need to adjust the z coordinate to zero, but do all three.

        add     tabup, tab
        cmp     bf, xzr

        ldp     x0, x1, [tabup]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc]
        ldp     x0, x1, [tabup, #16]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+16]
        ldp     x0, x1, [tabup, #32]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+32]
        ldp     x0, x1, [tabup, #48]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+48]
        ldp     x0, x1, [tabup, #64]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+64]
        ldp     x0, x1, [tabup, #80]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+80]
        ldp     x0, x1, [tabup, #96]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+96]
        ldp     x0, x1, [tabup, #112]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+112]
        ldp     x0, x1, [tabup, #128]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+128]
        ldp     x0, x1, [tabup, #144]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+144]
        ldp     x0, x1, [tabup, #160]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+160]
        ldp     x0, x1, [tabup, #176]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+176]
        ldp     x0, x1, [tabup, #192]
        csel    x0, x0, xzr, ne
        csel    x1, x1, xzr, ne
        stp     x0, x1, [acc+192]
        ldr     x0, [tabup, #208]
        csel    x0, x0, xzr, ne
        str     x0, [acc+208]

// Main loop over size-5 bitfields: double 5 times then add signed digit
// At each stage we shift the scalar left by 5 bits so we can simply pick
// the top 5 bits as the bitfield, saving some fiddle over indexing.

        mov     j, #520

Lp521_jscalarmul_alt_mainloop:
        sub     j, j, #5

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp521_jscalarmul_alt_jdouble)

// Choose the bitfield and adjust it to sign and magnitude

        ldp     x0, x1, [scalarb]
        ldp     x2, x3, [scalarb+16]
        ldp     x4, x5, [scalarb+32]
        ldp     x6, x7, [scalarb+48]
        ldr     x8, [scalarb+64]
        lsr     bf, x8, #59
        extr    x8, x8, x7, #59
        extr    x7, x7, x6, #59
        extr    x6, x6, x5, #59
        extr    x5, x5, x4, #59
        extr    x4, x4, x3, #59
        extr    x3, x3, x2, #59
        extr    x2, x2, x1, #59
        extr    x1, x1, x0, #59
        lsl     x0, x0, #5
        stp     x0, x1, [scalarb]
        stp     x2, x3, [scalarb+16]
        stp     x4, x5, [scalarb+32]
        stp     x6, x7, [scalarb+48]
        str     x8, [scalarb+64]

        subs    bf, bf, #16
        csetm   sgn, lo                 // sgn = sign of digit (1 = negative)
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
        add     tabup, tab
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
        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]
        stp     x4, x5, [tabent+32]
        stp     x6, x7, [tabent+48]
        str     x8, [tabent+64]

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, xzr
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        add     tabup, tab+2*NUMSIZE
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
        stp     x0, x1, [tabent+2*NUMSIZE]
        stp     x2, x3, [tabent+2*NUMSIZE+16]
        stp     x4, x5, [tabent+2*NUMSIZE+32]
        stp     x6, x7, [tabent+2*NUMSIZE+48]
        str     x8, [tabent+2*NUMSIZE+64]

        mov     x0, xzr
        mov     x1, xzr
        mov     x2, xzr
        mov     x3, xzr
        mov     x4, xzr
        mov     x5, xzr
        mov     x6, xzr
        mov     x7, xzr
        mov     x8, xzr
        add     tabup, tab+NUMSIZE
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
// This is done carefully to give coordinates < p_521 even in
// the degenerate case y = 0 (when z = 0 for points on the curve).

        orr     x10, x0, x1
        orr     x11, x2, x3
        orr     x12, x4, x5
        orr     x13, x6, x7
        orr     x10, x10, x11
        orr     x12, x12, x13
        orr     x12, x12, x8
        orr     x10, x10, x12
        cmp     x10, xzr
        csel    sgn, sgn, xzr, ne

        eor     x0, x0, sgn
        eor     x1, x1, sgn
        eor     x2, x2, sgn
        eor     x3, x3, sgn
        eor     x4, x4, sgn
        eor     x5, x5, sgn
        eor     x6, x6, sgn
        eor     x7, x7, sgn
        and     sgn, sgn, #0x1FF
        eor     x8, x8, sgn

        stp     x0, x1, [tabent+NUMSIZE]
        stp     x2, x3, [tabent+NUMSIZE+16]
        stp     x4, x5, [tabent+NUMSIZE+32]
        stp     x6, x7, [tabent+NUMSIZE+48]
        str     x8, [tabent+NUMSIZE+64]

// Add to the accumulator

        add     x0, acc
        add     x1, acc
        add     x2, tabent
        CFI_BL(Lp521_jscalarmul_alt_jadd)

        cbnz    j, Lp521_jscalarmul_alt_mainloop

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
        ldp     x0, x1, [acc+144]
        stp     x0, x1, [res, #144]
        ldp     x0, x1, [acc+160]
        stp     x0, x1, [res, #160]
        ldp     x0, x1, [acc+176]
        stp     x0, x1, [res, #176]
        ldp     x0, x1, [acc+192]
        stp     x0, x1, [res, #192]
        ldr     x0, [acc+208]
        str     x0, [res, #208]

// Restore stack and registers and return

        CFI_INC_SP(NSPACE)
        CFI_POP2(x21,x30)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p521_jscalarmul_alt)

// Local copies of subroutines, complete clones at the moment except
// that we share multiplication and squaring between the point operations.

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_alt_bignum_mod_p521_9)

Lp521_jscalarmul_alt_bignum_mod_p521_9:
        CFI_START
        ldr     x12, [x1, #64]
        lsr     x2, x12, #9
        cmp     xzr, xzr
        ldp     x4, x5, [x1]
        adcs    xzr, x4, x2
        adcs    xzr, x5, xzr
        ldp     x6, x7, [x1, #16]
        and     x3, x6, x7
        adcs    xzr, x3, xzr
        ldp     x8, x9, [x1, #32]
        and     x3, x8, x9
        adcs    xzr, x3, xzr
        ldp     x10, x11, [x1, #48]
        and     x3, x10, x11
        adcs    xzr, x3, xzr
        orr     x3, x12, #0xfffffffffffffe00
        adcs    x3, x3, xzr
        adcs    x4, x4, x2
        adcs    x5, x5, xzr
        adcs    x6, x6, xzr
        adcs    x7, x7, xzr
        adcs    x8, x8, xzr
        adcs    x9, x9, xzr
        adcs    x10, x10, xzr
        adcs    x11, x11, xzr
        adc     x12, x12, xzr
        and     x12, x12, #0x1ff
        stp     x4, x5, [x0]
        stp     x6, x7, [x0, #16]
        stp     x8, x9, [x0, #32]
        stp     x10, x11, [x0, #48]
        str     x12, [x0, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_alt_bignum_mod_p521_9)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_alt_bignum_mod_n521_9)

Lp521_jscalarmul_alt_bignum_mod_n521_9:
        CFI_START
        ldr     x14, [x1, #64]
        lsr     x15, x14, #9
        add     x15, x15, #1
        mov     x2, #39927
        movk    x2, #28359, lsl #16
        movk    x2, #18657, lsl #32
        movk    x2, #17552, lsl #48
        mul     x6, x2, x15
        mov     x3, #47185
        movk    x3, #30307, lsl #16
        movk    x3, #13895, lsl #32
        movk    x3, #50250, lsl #48
        mul     x7, x3, x15
        mov     x4, #23087
        movk    x4, #2294, lsl #16
        movk    x4, #65207, lsl #32
        movk    x4, #32819, lsl #48
        mul     x8, x4, x15
        mov     x5, #27028
        movk    x5, #16592, lsl #16
        movk    x5, #30844, lsl #32
        movk    x5, #44665, lsl #48
        mul     x9, x5, x15
        lsl     x10, x15, #2
        add     x10, x10, x15
        umulh   x13, x2, x15
        adds    x7, x7, x13
        umulh   x13, x3, x15
        adcs    x8, x8, x13
        umulh   x13, x4, x15
        adcs    x9, x9, x13
        umulh   x13, x5, x15
        adc     x10, x10, x13
        ldp     x12, x13, [x1]
        adds    x6, x6, x12
        adcs    x7, x7, x13
        ldp     x12, x13, [x1, #16]
        adcs    x8, x8, x12
        adcs    x9, x9, x13
        ldp     x13, x11, [x1, #32]
        adcs    x10, x10, x13
        adcs    x11, x11, xzr
        ldp     x12, x13, [x1, #48]
        adcs    x12, x12, xzr
        adcs    x13, x13, xzr
        orr     x14, x14, #0xfffffffffffffe00
        adcs    x14, x14, xzr
        csetm   x15, lo
        and     x2, x2, x15
        subs    x6, x6, x2
        and     x3, x3, x15
        sbcs    x7, x7, x3
        and     x4, x4, x15
        sbcs    x8, x8, x4
        and     x5, x5, x15
        sbcs    x9, x9, x5
        mov     x2, #5
        and     x2, x2, x15
        sbcs    x10, x10, x2
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbc     x14, x14, xzr
        and     x14, x14, #0x1ff
        stp     x6, x7, [x0]
        stp     x8, x9, [x0, #16]
        stp     x10, x11, [x0, #32]
        stp     x12, x13, [x0, #48]
        str     x14, [x0, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_alt_bignum_mod_n521_9)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_alt_jadd)

Lp521_jscalarmul_alt_jadd:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_PUSH2(x29,x30)
        CFI_DEC_SP(576)
        mov     x27, x0
        mov     x28, x1
        mov     x29, x2
        mov     x0, sp
        add     x1, x28, #0x90
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        add     x0, sp, #0x168
        add     x1, x29, #0x90
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        add     x0, sp, #0x1f8
        add     x1, x29, #0x90
        add     x2, x28, #0x48
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x48
        add     x1, x28, #0x90
        add     x2, x29, #0x48
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x90
        mov     x1, sp
        add     x2, x29, #0x0
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x120
        add     x1, sp, #0x168
        add     x2, x28, #0x0
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x48
        mov     x1, sp
        add     x2, sp, #0x48
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x1f8
        add     x1, sp, #0x168
        add     x2, sp, #0x1f8
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x168
        add     x1, sp, #0x90
        add     x2, sp, #0x120
        CFI_BL(Lp521_jscalarmul_alt_sub_p521)
        add     x0, sp, #0x48
        add     x1, sp, #0x48
        add     x2, sp, #0x1f8
        CFI_BL(Lp521_jscalarmul_alt_sub_p521)
        add     x0, sp, #0xd8
        add     x1, sp, #0x168
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        mov     x0, sp
        add     x1, sp, #0x48
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        add     x0, sp, #0x120
        add     x1, sp, #0xd8
        add     x2, sp, #0x120
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x90
        add     x1, sp, #0xd8
        add     x2, sp, #0x90
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        mov     x0, sp
        mov     x1, sp
        add     x2, sp, #0x120
        CFI_BL(Lp521_jscalarmul_alt_sub_p521)
        add     x0, sp, #0xd8
        add     x1, sp, #0x90
        add     x2, sp, #0x120
        CFI_BL(Lp521_jscalarmul_alt_sub_p521)
        add     x0, sp, #0x168
        add     x1, sp, #0x168
        add     x2, x28, #0x90
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        mov     x0, sp
        mov     x1, sp
        add     x2, sp, #0x90
        CFI_BL(Lp521_jscalarmul_alt_sub_p521)
        add     x0, sp, #0x120
        add     x1, sp, #0x120
        mov     x2, sp
        CFI_BL(Lp521_jscalarmul_alt_sub_p521)
        add     x0, sp, #0xd8
        add     x1, sp, #0xd8
        add     x2, sp, #0x1f8
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x168
        add     x1, sp, #0x168
        add     x2, x29, #0x90
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x120
        add     x1, sp, #0x48
        add     x2, sp, #0x120
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x120
        add     x1, sp, #0x120
        add     x2, sp, #0xd8
        CFI_BL(Lp521_jscalarmul_alt_sub_p521)
        ldp     x0, x1, [x28, #144]
        ldp     x2, x3, [x28, #160]
        ldp     x4, x5, [x28, #176]
        ldp     x6, x7, [x28, #192]
        ldr     x8, [x28, #208]
        orr     x20, x0, x1
        orr     x21, x2, x3
        orr     x22, x4, x5
        orr     x23, x6, x7
        orr     x20, x20, x21
        orr     x22, x22, x23
        orr     x20, x20, x8
        orr     x20, x20, x22
        cmp     x20, xzr
        cset    x20, ne
        ldp     x10, x11, [x29, #144]
        ldp     x12, x13, [x29, #160]
        ldp     x14, x15, [x29, #176]
        ldp     x16, x17, [x29, #192]
        ldr     x19, [x29, #208]
        orr     x21, x10, x11
        orr     x22, x12, x13
        orr     x23, x14, x15
        orr     x24, x16, x17
        orr     x21, x21, x22
        orr     x23, x23, x24
        orr     x21, x21, x19
        orr     x21, x21, x23
        csel    x0, x0, x10, ne
        csel    x1, x1, x11, ne
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        csel    x4, x4, x14, ne
        csel    x5, x5, x15, ne
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        csel    x8, x8, x19, ne
        cmp     x21, xzr
        cset    x21, ne
        cmp     x21, x20
        ldp     x10, x11, [sp, #360]
        ldp     x12, x13, [sp, #376]
        ldp     x14, x15, [sp, #392]
        ldp     x16, x17, [sp, #408]
        ldr     x19, [sp, #424]
        csel    x0, x0, x10, ne
        csel    x1, x1, x11, ne
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        csel    x4, x4, x14, ne
        csel    x5, x5, x15, ne
        csel    x6, x6, x16, ne
        csel    x7, x7, x17, ne
        csel    x8, x8, x19, ne
        stp     x0, x1, [sp, #360]
        stp     x2, x3, [sp, #376]
        stp     x4, x5, [sp, #392]
        stp     x6, x7, [sp, #408]
        str     x8, [sp, #424]
        ldp     x20, x21, [x28]
        ldp     x0, x1, [sp]
        csel    x0, x20, x0, cc
        csel    x1, x21, x1, cc
        ldp     x20, x21, [x29]
        csel    x0, x20, x0, hi
        csel    x1, x21, x1, hi
        ldp     x20, x21, [x28, #16]
        ldp     x2, x3, [sp, #16]
        csel    x2, x20, x2, cc
        csel    x3, x21, x3, cc
        ldp     x20, x21, [x29, #16]
        csel    x2, x20, x2, hi
        csel    x3, x21, x3, hi
        ldp     x20, x21, [x28, #32]
        ldp     x4, x5, [sp, #32]
        csel    x4, x20, x4, cc
        csel    x5, x21, x5, cc
        ldp     x20, x21, [x29, #32]
        csel    x4, x20, x4, hi
        csel    x5, x21, x5, hi
        ldp     x20, x21, [x28, #48]
        ldp     x6, x7, [sp, #48]
        csel    x6, x20, x6, cc
        csel    x7, x21, x7, cc
        ldp     x20, x21, [x29, #48]
        csel    x6, x20, x6, hi
        csel    x7, x21, x7, hi
        ldr     x20, [x28, #64]
        ldr     x8, [sp, #64]
        csel    x8, x20, x8, cc
        ldr     x21, [x29, #64]
        csel    x8, x21, x8, hi
        ldp     x20, x21, [x28, #72]
        ldp     x10, x11, [sp, #288]
        csel    x10, x20, x10, cc
        csel    x11, x21, x11, cc
        ldp     x20, x21, [x29, #72]
        csel    x10, x20, x10, hi
        csel    x11, x21, x11, hi
        ldp     x20, x21, [x28, #88]
        ldp     x12, x13, [sp, #304]
        csel    x12, x20, x12, cc
        csel    x13, x21, x13, cc
        ldp     x20, x21, [x29, #88]
        csel    x12, x20, x12, hi
        csel    x13, x21, x13, hi
        ldp     x20, x21, [x28, #104]
        ldp     x14, x15, [sp, #320]
        csel    x14, x20, x14, cc
        csel    x15, x21, x15, cc
        ldp     x20, x21, [x29, #104]
        csel    x14, x20, x14, hi
        csel    x15, x21, x15, hi
        ldp     x20, x21, [x28, #120]
        ldp     x16, x17, [sp, #336]
        csel    x16, x20, x16, cc
        csel    x17, x21, x17, cc
        ldp     x20, x21, [x29, #120]
        csel    x16, x20, x16, hi
        csel    x17, x21, x17, hi
        ldr     x20, [x28, #136]
        ldr     x19, [sp, #352]
        csel    x19, x20, x19, cc
        ldr     x21, [x29, #136]
        csel    x19, x21, x19, hi
        stp     x0, x1, [x27]
        stp     x2, x3, [x27, #16]
        stp     x4, x5, [x27, #32]
        stp     x6, x7, [x27, #48]
        str     x8, [x27, #64]
        ldp     x0, x1, [sp, #360]
        ldp     x2, x3, [sp, #376]
        ldp     x4, x5, [sp, #392]
        ldp     x6, x7, [sp, #408]
        ldr     x8, [sp, #424]
        stp     x10, x11, [x27, #72]
        stp     x12, x13, [x27, #88]
        stp     x14, x15, [x27, #104]
        stp     x16, x17, [x27, #120]
        str     x19, [x27, #136]
        stp     x0, x1, [x27, #144]
        stp     x2, x3, [x27, #160]
        stp     x4, x5, [x27, #176]
        stp     x6, x7, [x27, #192]
        str     x8, [x27, #208]
        CFI_INC_SP(576)
        CFI_POP2(x29,x30)
        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_alt_jadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_alt_jdouble)

Lp521_jscalarmul_alt_jdouble:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_PUSH2(x29,x30)
        CFI_DEC_SP(512)
        mov     x27, x0
        mov     x28, x1
        mov     x0, sp
        add     x1, x28, #0x90
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        add     x0, sp, #0x48
        add     x1, x28, #0x48
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        ldp     x5, x6, [x28]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x28, #16]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [x28, #32]
        ldp     x4, x3, [sp, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        ldp     x11, x12, [x28, #48]
        ldp     x4, x3, [sp, #48]
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        ldr     x13, [x28, #64]
        ldr     x4, [sp, #64]
        sbcs    x13, x13, x4
        sbcs    x5, x5, xzr
        sbcs    x6, x6, xzr
        sbcs    x7, x7, xzr
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        and     x13, x13, #0x1ff
        stp     x5, x6, [sp, #216]
        stp     x7, x8, [sp, #232]
        stp     x9, x10, [sp, #248]
        stp     x11, x12, [sp, #264]
        str     x13, [sp, #280]
        cmp     xzr, xzr
        ldp     x5, x6, [x28]
        ldp     x4, x3, [sp]
        adcs    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x28, #16]
        ldp     x4, x3, [sp, #16]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        ldp     x9, x10, [x28, #32]
        ldp     x4, x3, [sp, #32]
        adcs    x9, x9, x4
        adcs    x10, x10, x3
        ldp     x11, x12, [x28, #48]
        ldp     x4, x3, [sp, #48]
        adcs    x11, x11, x4
        adcs    x12, x12, x3
        ldr     x13, [x28, #64]
        ldr     x4, [sp, #64]
        adc     x13, x13, x4
        subs    x4, x13, #0x200
        csetm   x4, cs
        sbcs    x5, x5, xzr
        and     x4, x4, #0x200
        sbcs    x6, x6, xzr
        sbcs    x7, x7, xzr
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, x4
        stp     x5, x6, [sp, #144]
        stp     x7, x8, [sp, #160]
        stp     x9, x10, [sp, #176]
        stp     x11, x12, [sp, #192]
        str     x13, [sp, #208]
        add     x0, sp, #0xd8
        add     x1, sp, #0x90
        add     x2, sp, #0xd8
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        cmp     xzr, xzr
        ldp     x5, x6, [x28, #72]
        ldp     x4, x3, [x28, #144]
        adcs    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x28, #88]
        ldp     x4, x3, [x28, #160]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        ldp     x9, x10, [x28, #104]
        ldp     x4, x3, [x28, #176]
        adcs    x9, x9, x4
        adcs    x10, x10, x3
        ldp     x11, x12, [x28, #120]
        ldp     x4, x3, [x28, #192]
        adcs    x11, x11, x4
        adcs    x12, x12, x3
        ldr     x13, [x28, #136]
        ldr     x4, [x28, #208]
        adc     x13, x13, x4
        subs    x4, x13, #0x200
        csetm   x4, cs
        sbcs    x5, x5, xzr
        and     x4, x4, #0x200
        sbcs    x6, x6, xzr
        sbcs    x7, x7, xzr
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, x4
        stp     x5, x6, [sp, #144]
        stp     x7, x8, [sp, #160]
        stp     x9, x10, [sp, #176]
        stp     x11, x12, [sp, #192]
        str     x13, [sp, #208]
        add     x0, sp, #0x120
        add     x1, x28, #0x0
        add     x2, sp, #0x48
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        add     x0, sp, #0x168
        add     x1, sp, #0xd8
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        add     x0, sp, #0x90
        add     x1, sp, #0x90
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        ldp     x6, x7, [sp, #288]
        mov     x1, #0xc
        mul     x3, x1, x6
        mul     x4, x1, x7
        umulh   x6, x1, x6
        adds    x4, x4, x6
        umulh   x7, x1, x7
        ldp     x8, x9, [sp, #304]
        mul     x5, x1, x8
        mul     x6, x1, x9
        umulh   x8, x1, x8
        adcs    x5, x5, x7
        umulh   x9, x1, x9
        adcs    x6, x6, x8
        ldp     x10, x11, [sp, #320]
        mul     x7, x1, x10
        mul     x8, x1, x11
        umulh   x10, x1, x10
        adcs    x7, x7, x9
        umulh   x11, x1, x11
        adcs    x8, x8, x10
        ldp     x12, x13, [sp, #336]
        mul     x9, x1, x12
        mul     x10, x1, x13
        umulh   x12, x1, x12
        adcs    x9, x9, x11
        umulh   x13, x1, x13
        adcs    x10, x10, x12
        ldr     x14, [sp, #352]
        mul     x11, x1, x14
        adc     x11, x11, x13
        mov     x1, #0x9
        ldp     x20, x21, [sp, #360]
        mvn     x20, x20
        mul     x0, x1, x20
        umulh   x20, x1, x20
        adds    x3, x3, x0
        mvn     x21, x21
        mul     x0, x1, x21
        umulh   x21, x1, x21
        adcs    x4, x4, x0
        ldp     x22, x23, [sp, #376]
        mvn     x22, x22
        mul     x0, x1, x22
        umulh   x22, x1, x22
        adcs    x5, x5, x0
        mvn     x23, x23
        mul     x0, x1, x23
        umulh   x23, x1, x23
        adcs    x6, x6, x0
        ldp     x17, x19, [sp, #392]
        mvn     x17, x17
        mul     x0, x1, x17
        umulh   x17, x1, x17
        adcs    x7, x7, x0
        mvn     x19, x19
        mul     x0, x1, x19
        umulh   x19, x1, x19
        adcs    x8, x8, x0
        ldp     x2, x16, [sp, #408]
        mvn     x2, x2
        mul     x0, x1, x2
        umulh   x2, x1, x2
        adcs    x9, x9, x0
        mvn     x16, x16
        mul     x0, x1, x16
        umulh   x16, x1, x16
        adcs    x10, x10, x0
        ldr     x0, [sp, #424]
        eor     x0, x0, #0x1ff
        mul     x0, x1, x0
        adc     x11, x11, x0
        adds    x4, x4, x20
        adcs    x5, x5, x21
        and     x15, x4, x5
        adcs    x6, x6, x22
        and     x15, x15, x6
        adcs    x7, x7, x23
        and     x15, x15, x7
        adcs    x8, x8, x17
        and     x15, x15, x8
        adcs    x9, x9, x19
        and     x15, x15, x9
        adcs    x10, x10, x2
        and     x15, x15, x10
        adc     x11, x11, x16
        lsr     x12, x11, #9
        orr     x11, x11, #0xfffffffffffffe00
        cmp     xzr, xzr
        adcs    xzr, x3, x12
        adcs    xzr, x15, xzr
        adcs    xzr, x11, xzr
        adcs    x3, x3, x12
        adcs    x4, x4, xzr
        adcs    x5, x5, xzr
        adcs    x6, x6, xzr
        adcs    x7, x7, xzr
        adcs    x8, x8, xzr
        adcs    x9, x9, xzr
        adcs    x10, x10, xzr
        adc     x11, x11, xzr
        and     x11, x11, #0x1ff
        stp     x3, x4, [sp, #360]
        stp     x5, x6, [sp, #376]
        stp     x7, x8, [sp, #392]
        stp     x9, x10, [sp, #408]
        str     x11, [sp, #424]
        ldp     x5, x6, [sp, #144]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #160]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #176]
        ldp     x4, x3, [sp, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        ldp     x11, x12, [sp, #192]
        ldp     x4, x3, [sp, #48]
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        ldr     x13, [sp, #208]
        ldr     x4, [sp, #64]
        sbcs    x13, x13, x4
        sbcs    x5, x5, xzr
        sbcs    x6, x6, xzr
        sbcs    x7, x7, xzr
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        and     x13, x13, #0x1ff
        stp     x5, x6, [sp, #144]
        stp     x7, x8, [sp, #160]
        stp     x9, x10, [sp, #176]
        stp     x11, x12, [sp, #192]
        str     x13, [sp, #208]
        mov     x0, sp
        add     x1, sp, #0x48
        CFI_BL(Lp521_jscalarmul_alt_sqr_p521)
        add     x0, sp, #0xd8
        add     x1, sp, #0x168
        add     x2, sp, #0xd8
        CFI_BL(Lp521_jscalarmul_alt_mul_p521)
        ldp     x5, x6, [sp, #144]
        ldp     x4, x3, [sp, #72]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #160]
        ldp     x4, x3, [sp, #88]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #176]
        ldp     x4, x3, [sp, #104]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        ldp     x11, x12, [sp, #192]
        ldp     x4, x3, [sp, #120]
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        ldr     x13, [sp, #208]
        ldr     x4, [sp, #136]
        sbcs    x13, x13, x4
        sbcs    x5, x5, xzr
        sbcs    x6, x6, xzr
        sbcs    x7, x7, xzr
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        and     x13, x13, #0x1ff
        stp     x5, x6, [x27, #144]
        stp     x7, x8, [x27, #160]
        stp     x9, x10, [x27, #176]
        stp     x11, x12, [x27, #192]
        str     x13, [x27, #208]
        ldp     x6, x7, [sp, #288]
        lsl     x3, x6, #2
        extr    x4, x7, x6, #62
        ldp     x8, x9, [sp, #304]
        extr    x5, x8, x7, #62
        extr    x6, x9, x8, #62
        ldp     x10, x11, [sp, #320]
        extr    x7, x10, x9, #62
        extr    x8, x11, x10, #62
        ldp     x12, x13, [sp, #336]
        extr    x9, x12, x11, #62
        extr    x10, x13, x12, #62
        ldr     x14, [sp, #352]
        extr    x11, x14, x13, #62
        ldp     x0, x1, [sp, #360]
        mvn     x0, x0
        adds    x3, x3, x0
        sbcs    x4, x4, x1
        ldp     x0, x1, [sp, #376]
        sbcs    x5, x5, x0
        and     x15, x4, x5
        sbcs    x6, x6, x1
        and     x15, x15, x6
        ldp     x0, x1, [sp, #392]
        sbcs    x7, x7, x0
        and     x15, x15, x7
        sbcs    x8, x8, x1
        and     x15, x15, x8
        ldp     x0, x1, [sp, #408]
        sbcs    x9, x9, x0
        and     x15, x15, x9
        sbcs    x10, x10, x1
        and     x15, x15, x10
        ldr     x0, [sp, #424]
        eor     x0, x0, #0x1ff
        adc     x11, x11, x0
        lsr     x12, x11, #9
        orr     x11, x11, #0xfffffffffffffe00
        cmp     xzr, xzr
        adcs    xzr, x3, x12
        adcs    xzr, x15, xzr
        adcs    xzr, x11, xzr
        adcs    x3, x3, x12
        adcs    x4, x4, xzr
        adcs    x5, x5, xzr
        adcs    x6, x6, xzr
        adcs    x7, x7, xzr
        adcs    x8, x8, xzr
        adcs    x9, x9, xzr
        adcs    x10, x10, xzr
        adc     x11, x11, xzr
        and     x11, x11, #0x1ff
        stp     x3, x4, [x27]
        stp     x5, x6, [x27, #16]
        stp     x7, x8, [x27, #32]
        stp     x9, x10, [x27, #48]
        str     x11, [x27, #64]
        ldp     x6, x7, [sp, #216]
        lsl     x3, x6, #1
        adds    x3, x3, x6
        extr    x4, x7, x6, #63
        adcs    x4, x4, x7
        ldp     x8, x9, [sp, #232]
        extr    x5, x8, x7, #63
        adcs    x5, x5, x8
        extr    x6, x9, x8, #63
        adcs    x6, x6, x9
        ldp     x10, x11, [sp, #248]
        extr    x7, x10, x9, #63
        adcs    x7, x7, x10
        extr    x8, x11, x10, #63
        adcs    x8, x8, x11
        ldp     x12, x13, [sp, #264]
        extr    x9, x12, x11, #63
        adcs    x9, x9, x12
        extr    x10, x13, x12, #63
        adcs    x10, x10, x13
        ldr     x14, [sp, #280]
        extr    x11, x14, x13, #63
        adc     x11, x11, x14
        ldp     x20, x21, [sp]
        mvn     x20, x20
        lsl     x0, x20, #3
        adds    x3, x3, x0
        mvn     x21, x21
        extr    x0, x21, x20, #61
        adcs    x4, x4, x0
        ldp     x22, x23, [sp, #16]
        mvn     x22, x22
        extr    x0, x22, x21, #61
        adcs    x5, x5, x0
        and     x15, x4, x5
        mvn     x23, x23
        extr    x0, x23, x22, #61
        adcs    x6, x6, x0
        and     x15, x15, x6
        ldp     x20, x21, [sp, #32]
        mvn     x20, x20
        extr    x0, x20, x23, #61
        adcs    x7, x7, x0
        and     x15, x15, x7
        mvn     x21, x21
        extr    x0, x21, x20, #61
        adcs    x8, x8, x0
        and     x15, x15, x8
        ldp     x22, x23, [sp, #48]
        mvn     x22, x22
        extr    x0, x22, x21, #61
        adcs    x9, x9, x0
        and     x15, x15, x9
        mvn     x23, x23
        extr    x0, x23, x22, #61
        adcs    x10, x10, x0
        and     x15, x15, x10
        ldr     x0, [sp, #64]
        eor     x0, x0, #0x1ff
        extr    x0, x0, x23, #61
        adc     x11, x11, x0
        lsr     x12, x11, #9
        orr     x11, x11, #0xfffffffffffffe00
        cmp     xzr, xzr
        adcs    xzr, x3, x12
        adcs    xzr, x15, xzr
        adcs    xzr, x11, xzr
        adcs    x3, x3, x12
        adcs    x4, x4, xzr
        adcs    x5, x5, xzr
        adcs    x6, x6, xzr
        adcs    x7, x7, xzr
        adcs    x8, x8, xzr
        adcs    x9, x9, xzr
        adcs    x10, x10, xzr
        adc     x11, x11, xzr
        and     x11, x11, #0x1ff
        stp     x3, x4, [x27, #72]
        stp     x5, x6, [x27, #88]
        stp     x7, x8, [x27, #104]
        stp     x9, x10, [x27, #120]
        str     x11, [x27, #136]
        CFI_INC_SP(512)
        CFI_POP2(x29,x30)
        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_alt_jdouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_alt_mul_p521)

Lp521_jscalarmul_alt_mul_p521:
        CFI_START
        ldp     x3, x4, [x1]
        ldp     x5, x6, [x2]
        mul     x15, x3, x5
        umulh   x16, x3, x5
        mul     x14, x3, x6
        umulh   x17, x3, x6
        adds    x16, x16, x14
        ldp     x7, x8, [x2, #16]
        mul     x14, x3, x7
        umulh   x19, x3, x7
        adcs    x17, x17, x14
        mul     x14, x3, x8
        umulh   x20, x3, x8
        adcs    x19, x19, x14
        ldp     x9, x10, [x2, #32]
        mul     x14, x3, x9
        umulh   x21, x3, x9
        adcs    x20, x20, x14
        mul     x14, x3, x10
        umulh   x22, x3, x10
        adcs    x21, x21, x14
        ldp     x11, x12, [x2, #48]
        mul     x14, x3, x11
        umulh   x23, x3, x11
        adcs    x22, x22, x14
        ldr     x13, [x2, #64]
        mul     x14, x3, x12
        umulh   x24, x3, x12
        adcs    x23, x23, x14
        mul     x14, x3, x13
        umulh   x25, x3, x13
        adcs    x24, x24, x14
        adc     x25, x25, xzr
        mul     x14, x4, x5
        adds    x16, x16, x14
        mul     x14, x4, x6
        adcs    x17, x17, x14
        mul     x14, x4, x7
        adcs    x19, x19, x14
        mul     x14, x4, x8
        adcs    x20, x20, x14
        mul     x14, x4, x9
        adcs    x21, x21, x14
        mul     x14, x4, x10
        adcs    x22, x22, x14
        mul     x14, x4, x11
        adcs    x23, x23, x14
        mul     x14, x4, x12
        adcs    x24, x24, x14
        mul     x14, x4, x13
        adcs    x25, x25, x14
        cset    x26, cs
        umulh   x14, x4, x5
        adds    x17, x17, x14
        umulh   x14, x4, x6
        adcs    x19, x19, x14
        umulh   x14, x4, x7
        adcs    x20, x20, x14
        umulh   x14, x4, x8
        adcs    x21, x21, x14
        umulh   x14, x4, x9
        adcs    x22, x22, x14
        umulh   x14, x4, x10
        adcs    x23, x23, x14
        umulh   x14, x4, x11
        adcs    x24, x24, x14
        umulh   x14, x4, x12
        adcs    x25, x25, x14
        umulh   x14, x4, x13
        adc     x26, x26, x14
        stp     x15, x16, [sp, #432]
        ldp     x3, x4, [x1, #16]
        mul     x14, x3, x5
        adds    x17, x17, x14
        mul     x14, x3, x6
        adcs    x19, x19, x14
        mul     x14, x3, x7
        adcs    x20, x20, x14
        mul     x14, x3, x8
        adcs    x21, x21, x14
        mul     x14, x3, x9
        adcs    x22, x22, x14
        mul     x14, x3, x10
        adcs    x23, x23, x14
        mul     x14, x3, x11
        adcs    x24, x24, x14
        mul     x14, x3, x12
        adcs    x25, x25, x14
        mul     x14, x3, x13
        adcs    x26, x26, x14
        cset    x15, cs
        umulh   x14, x3, x5
        adds    x19, x19, x14
        umulh   x14, x3, x6
        adcs    x20, x20, x14
        umulh   x14, x3, x7
        adcs    x21, x21, x14
        umulh   x14, x3, x8
        adcs    x22, x22, x14
        umulh   x14, x3, x9
        adcs    x23, x23, x14
        umulh   x14, x3, x10
        adcs    x24, x24, x14
        umulh   x14, x3, x11
        adcs    x25, x25, x14
        umulh   x14, x3, x12
        adcs    x26, x26, x14
        umulh   x14, x3, x13
        adc     x15, x15, x14
        mul     x14, x4, x5
        adds    x19, x19, x14
        mul     x14, x4, x6
        adcs    x20, x20, x14
        mul     x14, x4, x7
        adcs    x21, x21, x14
        mul     x14, x4, x8
        adcs    x22, x22, x14
        mul     x14, x4, x9
        adcs    x23, x23, x14
        mul     x14, x4, x10
        adcs    x24, x24, x14
        mul     x14, x4, x11
        adcs    x25, x25, x14
        mul     x14, x4, x12
        adcs    x26, x26, x14
        mul     x14, x4, x13
        adcs    x15, x15, x14
        cset    x16, cs
        umulh   x14, x4, x5
        adds    x20, x20, x14
        umulh   x14, x4, x6
        adcs    x21, x21, x14
        umulh   x14, x4, x7
        adcs    x22, x22, x14
        umulh   x14, x4, x8
        adcs    x23, x23, x14
        umulh   x14, x4, x9
        adcs    x24, x24, x14
        umulh   x14, x4, x10
        adcs    x25, x25, x14
        umulh   x14, x4, x11
        adcs    x26, x26, x14
        umulh   x14, x4, x12
        adcs    x15, x15, x14
        umulh   x14, x4, x13
        adc     x16, x16, x14
        stp     x17, x19, [sp, #448]
        ldp     x3, x4, [x1, #32]
        mul     x14, x3, x5
        adds    x20, x20, x14
        mul     x14, x3, x6
        adcs    x21, x21, x14
        mul     x14, x3, x7
        adcs    x22, x22, x14
        mul     x14, x3, x8
        adcs    x23, x23, x14
        mul     x14, x3, x9
        adcs    x24, x24, x14
        mul     x14, x3, x10
        adcs    x25, x25, x14
        mul     x14, x3, x11
        adcs    x26, x26, x14
        mul     x14, x3, x12
        adcs    x15, x15, x14
        mul     x14, x3, x13
        adcs    x16, x16, x14
        cset    x17, cs
        umulh   x14, x3, x5
        adds    x21, x21, x14
        umulh   x14, x3, x6
        adcs    x22, x22, x14
        umulh   x14, x3, x7
        adcs    x23, x23, x14
        umulh   x14, x3, x8
        adcs    x24, x24, x14
        umulh   x14, x3, x9
        adcs    x25, x25, x14
        umulh   x14, x3, x10
        adcs    x26, x26, x14
        umulh   x14, x3, x11
        adcs    x15, x15, x14
        umulh   x14, x3, x12
        adcs    x16, x16, x14
        umulh   x14, x3, x13
        adc     x17, x17, x14
        mul     x14, x4, x5
        adds    x21, x21, x14
        mul     x14, x4, x6
        adcs    x22, x22, x14
        mul     x14, x4, x7
        adcs    x23, x23, x14
        mul     x14, x4, x8
        adcs    x24, x24, x14
        mul     x14, x4, x9
        adcs    x25, x25, x14
        mul     x14, x4, x10
        adcs    x26, x26, x14
        mul     x14, x4, x11
        adcs    x15, x15, x14
        mul     x14, x4, x12
        adcs    x16, x16, x14
        mul     x14, x4, x13
        adcs    x17, x17, x14
        cset    x19, cs
        umulh   x14, x4, x5
        adds    x22, x22, x14
        umulh   x14, x4, x6
        adcs    x23, x23, x14
        umulh   x14, x4, x7
        adcs    x24, x24, x14
        umulh   x14, x4, x8
        adcs    x25, x25, x14
        umulh   x14, x4, x9
        adcs    x26, x26, x14
        umulh   x14, x4, x10
        adcs    x15, x15, x14
        umulh   x14, x4, x11
        adcs    x16, x16, x14
        umulh   x14, x4, x12
        adcs    x17, x17, x14
        umulh   x14, x4, x13
        adc     x19, x19, x14
        stp     x20, x21, [sp, #464]
        ldp     x3, x4, [x1, #48]
        mul     x14, x3, x5
        adds    x22, x22, x14
        mul     x14, x3, x6
        adcs    x23, x23, x14
        mul     x14, x3, x7
        adcs    x24, x24, x14
        mul     x14, x3, x8
        adcs    x25, x25, x14
        mul     x14, x3, x9
        adcs    x26, x26, x14
        mul     x14, x3, x10
        adcs    x15, x15, x14
        mul     x14, x3, x11
        adcs    x16, x16, x14
        mul     x14, x3, x12
        adcs    x17, x17, x14
        mul     x14, x3, x13
        adcs    x19, x19, x14
        cset    x20, cs
        umulh   x14, x3, x5
        adds    x23, x23, x14
        umulh   x14, x3, x6
        adcs    x24, x24, x14
        umulh   x14, x3, x7
        adcs    x25, x25, x14
        umulh   x14, x3, x8
        adcs    x26, x26, x14
        umulh   x14, x3, x9
        adcs    x15, x15, x14
        umulh   x14, x3, x10
        adcs    x16, x16, x14
        umulh   x14, x3, x11
        adcs    x17, x17, x14
        umulh   x14, x3, x12
        adcs    x19, x19, x14
        umulh   x14, x3, x13
        adc     x20, x20, x14
        mul     x14, x4, x5
        adds    x23, x23, x14
        mul     x14, x4, x6
        adcs    x24, x24, x14
        mul     x14, x4, x7
        adcs    x25, x25, x14
        mul     x14, x4, x8
        adcs    x26, x26, x14
        mul     x14, x4, x9
        adcs    x15, x15, x14
        mul     x14, x4, x10
        adcs    x16, x16, x14
        mul     x14, x4, x11
        adcs    x17, x17, x14
        mul     x14, x4, x12
        adcs    x19, x19, x14
        mul     x14, x4, x13
        adcs    x20, x20, x14
        cset    x21, cs
        umulh   x14, x4, x5
        adds    x24, x24, x14
        umulh   x14, x4, x6
        adcs    x25, x25, x14
        umulh   x14, x4, x7
        adcs    x26, x26, x14
        umulh   x14, x4, x8
        adcs    x15, x15, x14
        umulh   x14, x4, x9
        adcs    x16, x16, x14
        umulh   x14, x4, x10
        adcs    x17, x17, x14
        umulh   x14, x4, x11
        adcs    x19, x19, x14
        umulh   x14, x4, x12
        adcs    x20, x20, x14
        umulh   x14, x4, x13
        adc     x21, x21, x14
        stp     x22, x23, [sp, #480]
        ldr     x3, [x1, #64]
        mul     x14, x3, x5
        adds    x24, x24, x14
        mul     x14, x3, x6
        adcs    x25, x25, x14
        mul     x14, x3, x7
        adcs    x26, x26, x14
        mul     x14, x3, x8
        adcs    x15, x15, x14
        mul     x14, x3, x9
        adcs    x16, x16, x14
        mul     x14, x3, x10
        adcs    x17, x17, x14
        mul     x14, x3, x11
        adcs    x19, x19, x14
        mul     x14, x3, x12
        adcs    x20, x20, x14
        mul     x14, x3, x13
        adc     x21, x21, x14
        umulh   x14, x3, x5
        adds    x25, x25, x14
        umulh   x14, x3, x6
        adcs    x26, x26, x14
        umulh   x14, x3, x7
        adcs    x15, x15, x14
        umulh   x14, x3, x8
        adcs    x16, x16, x14
        umulh   x14, x3, x9
        adcs    x17, x17, x14
        umulh   x14, x3, x10
        adcs    x19, x19, x14
        umulh   x14, x3, x11
        adcs    x20, x20, x14
        umulh   x14, x3, x12
        adc     x21, x21, x14
        cmp     xzr, xzr
        ldp     x5, x6, [sp, #432]
        extr    x14, x25, x24, #9
        adcs    x5, x5, x14
        extr    x14, x26, x25, #9
        adcs    x6, x6, x14
        ldp     x7, x8, [sp, #448]
        extr    x14, x15, x26, #9
        adcs    x7, x7, x14
        extr    x14, x16, x15, #9
        adcs    x8, x8, x14
        ldp     x9, x10, [sp, #464]
        extr    x14, x17, x16, #9
        adcs    x9, x9, x14
        extr    x14, x19, x17, #9
        adcs    x10, x10, x14
        ldp     x11, x12, [sp, #480]
        extr    x14, x20, x19, #9
        adcs    x11, x11, x14
        extr    x14, x21, x20, #9
        adcs    x12, x12, x14
        orr     x13, x24, #0xfffffffffffffe00
        lsr     x14, x21, #9
        adcs    x13, x13, x14
        sbcs    x5, x5, xzr
        sbcs    x6, x6, xzr
        sbcs    x7, x7, xzr
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbc     x13, x13, xzr
        and     x13, x13, #0x1ff
        stp     x5, x6, [x0]
        stp     x7, x8, [x0, #16]
        stp     x9, x10, [x0, #32]
        stp     x11, x12, [x0, #48]
        str     x13, [x0, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_alt_mul_p521)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_alt_sqr_p521)

Lp521_jscalarmul_alt_sqr_p521:
        CFI_START
        ldp     x2, x3, [x1]
        mul     x11, x2, x3
        umulh   x12, x2, x3
        ldp     x4, x5, [x1, #16]
        mul     x10, x2, x4
        umulh   x13, x2, x4
        adds    x12, x12, x10
        ldp     x6, x7, [x1, #32]
        mul     x10, x2, x5
        umulh   x14, x2, x5
        adcs    x13, x13, x10
        ldp     x8, x9, [x1, #48]
        mul     x10, x2, x6
        umulh   x15, x2, x6
        adcs    x14, x14, x10
        mul     x10, x2, x7
        umulh   x16, x2, x7
        adcs    x15, x15, x10
        mul     x10, x2, x8
        umulh   x17, x2, x8
        adcs    x16, x16, x10
        mul     x10, x2, x9
        umulh   x19, x2, x9
        adcs    x17, x17, x10
        adc     x19, x19, xzr
        mul     x10, x3, x4
        adds    x13, x13, x10
        mul     x10, x3, x5
        adcs    x14, x14, x10
        mul     x10, x3, x6
        adcs    x15, x15, x10
        mul     x10, x3, x7
        adcs    x16, x16, x10
        mul     x10, x3, x8
        adcs    x17, x17, x10
        mul     x10, x3, x9
        adcs    x19, x19, x10
        cset    x20, cs
        umulh   x10, x3, x4
        adds    x14, x14, x10
        umulh   x10, x3, x5
        adcs    x15, x15, x10
        umulh   x10, x3, x6
        adcs    x16, x16, x10
        umulh   x10, x3, x7
        adcs    x17, x17, x10
        umulh   x10, x3, x8
        adcs    x19, x19, x10
        umulh   x10, x3, x9
        adc     x20, x20, x10
        mul     x10, x6, x7
        umulh   x21, x6, x7
        adds    x20, x20, x10
        adc     x21, x21, xzr
        mul     x10, x4, x5
        adds    x15, x15, x10
        mul     x10, x4, x6
        adcs    x16, x16, x10
        mul     x10, x4, x7
        adcs    x17, x17, x10
        mul     x10, x4, x8
        adcs    x19, x19, x10
        mul     x10, x4, x9
        adcs    x20, x20, x10
        mul     x10, x6, x8
        adcs    x21, x21, x10
        cset    x22, cs
        umulh   x10, x4, x5
        adds    x16, x16, x10
        umulh   x10, x4, x6
        adcs    x17, x17, x10
        umulh   x10, x4, x7
        adcs    x19, x19, x10
        umulh   x10, x4, x8
        adcs    x20, x20, x10
        umulh   x10, x4, x9
        adcs    x21, x21, x10
        umulh   x10, x6, x8
        adc     x22, x22, x10
        mul     x10, x7, x8
        umulh   x23, x7, x8
        adds    x22, x22, x10
        adc     x23, x23, xzr
        mul     x10, x5, x6
        adds    x17, x17, x10
        mul     x10, x5, x7
        adcs    x19, x19, x10
        mul     x10, x5, x8
        adcs    x20, x20, x10
        mul     x10, x5, x9
        adcs    x21, x21, x10
        mul     x10, x6, x9
        adcs    x22, x22, x10
        mul     x10, x7, x9
        adcs    x23, x23, x10
        cset    x24, cs
        umulh   x10, x5, x6
        adds    x19, x19, x10
        umulh   x10, x5, x7
        adcs    x20, x20, x10
        umulh   x10, x5, x8
        adcs    x21, x21, x10
        umulh   x10, x5, x9
        adcs    x22, x22, x10
        umulh   x10, x6, x9
        adcs    x23, x23, x10
        umulh   x10, x7, x9
        adc     x24, x24, x10
        mul     x10, x8, x9
        umulh   x25, x8, x9
        adds    x24, x24, x10
        adc     x25, x25, xzr
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        adcs    x15, x15, x15
        adcs    x16, x16, x16
        adcs    x17, x17, x17
        adcs    x19, x19, x19
        adcs    x20, x20, x20
        adcs    x21, x21, x21
        adcs    x22, x22, x22
        adcs    x23, x23, x23
        adcs    x24, x24, x24
        adcs    x25, x25, x25
        cset    x26, cs
        umulh   x10, x2, x2
        adds    x11, x11, x10
        mul     x10, x3, x3
        adcs    x12, x12, x10
        umulh   x10, x3, x3
        adcs    x13, x13, x10
        mul     x10, x4, x4
        adcs    x14, x14, x10
        umulh   x10, x4, x4
        adcs    x15, x15, x10
        mul     x10, x5, x5
        adcs    x16, x16, x10
        umulh   x10, x5, x5
        adcs    x17, x17, x10
        mul     x10, x6, x6
        adcs    x19, x19, x10
        umulh   x10, x6, x6
        adcs    x20, x20, x10
        mul     x10, x7, x7
        adcs    x21, x21, x10
        umulh   x10, x7, x7
        adcs    x22, x22, x10
        mul     x10, x8, x8
        adcs    x23, x23, x10
        umulh   x10, x8, x8
        adcs    x24, x24, x10
        mul     x10, x9, x9
        adcs    x25, x25, x10
        umulh   x10, x9, x9
        adc     x26, x26, x10
        ldr     x1, [x1, #64]
        add     x1, x1, x1
        mul     x10, x1, x2
        adds    x19, x19, x10
        umulh   x10, x1, x2
        adcs    x20, x20, x10
        mul     x10, x1, x4
        adcs    x21, x21, x10
        umulh   x10, x1, x4
        adcs    x22, x22, x10
        mul     x10, x1, x6
        adcs    x23, x23, x10
        umulh   x10, x1, x6
        adcs    x24, x24, x10
        mul     x10, x1, x8
        adcs    x25, x25, x10
        umulh   x10, x1, x8
        adcs    x26, x26, x10
        lsr     x4, x1, #1
        mul     x4, x4, x4
        adc     x4, x4, xzr
        mul     x10, x1, x3
        adds    x20, x20, x10
        umulh   x10, x1, x3
        adcs    x21, x21, x10
        mul     x10, x1, x5
        adcs    x22, x22, x10
        umulh   x10, x1, x5
        adcs    x23, x23, x10
        mul     x10, x1, x7
        adcs    x24, x24, x10
        umulh   x10, x1, x7
        adcs    x25, x25, x10
        mul     x10, x1, x9
        adcs    x26, x26, x10
        umulh   x10, x1, x9
        adc     x4, x4, x10
        mul     x2, x2, x2
        cmp     xzr, xzr
        extr    x10, x20, x19, #9
        adcs    x2, x2, x10
        extr    x10, x21, x20, #9
        adcs    x11, x11, x10
        extr    x10, x22, x21, #9
        adcs    x12, x12, x10
        extr    x10, x23, x22, #9
        adcs    x13, x13, x10
        extr    x10, x24, x23, #9
        adcs    x14, x14, x10
        extr    x10, x25, x24, #9
        adcs    x15, x15, x10
        extr    x10, x26, x25, #9
        adcs    x16, x16, x10
        extr    x10, x4, x26, #9
        adcs    x17, x17, x10
        orr     x19, x19, #0xfffffffffffffe00
        lsr     x10, x4, #9
        adcs    x19, x19, x10
        sbcs    x2, x2, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbcs    x15, x15, xzr
        sbcs    x16, x16, xzr
        sbcs    x17, x17, xzr
        sbc     x19, x19, xzr
        and     x19, x19, #0x1ff
        stp     x2, x11, [x0]
        stp     x12, x13, [x0, #16]
        stp     x14, x15, [x0, #32]
        stp     x16, x17, [x0, #48]
        str     x19, [x0, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_alt_sqr_p521)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_alt_sub_p521)

Lp521_jscalarmul_alt_sub_p521:
        CFI_START
        ldp     x5, x6, [x1]
        ldp     x4, x3, [x2]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x1, #16]
        ldp     x4, x3, [x2, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [x1, #32]
        ldp     x4, x3, [x2, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        ldp     x11, x12, [x1, #48]
        ldp     x4, x3, [x2, #48]
        sbcs    x11, x11, x4
        sbcs    x12, x12, x3
        ldr     x13, [x1, #64]
        ldr     x4, [x2, #64]
        sbcs    x13, x13, x4
        sbcs    x5, x5, xzr
        sbcs    x6, x6, xzr
        sbcs    x7, x7, xzr
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbcs    x10, x10, xzr
        sbcs    x11, x11, xzr
        sbcs    x12, x12, xzr
        sbcs    x13, x13, xzr
        and     x13, x13, #0x1ff
        stp     x5, x6, [x0]
        stp     x7, x8, [x0, #16]
        stp     x9, x10, [x0, #32]
        stp     x11, x12, [x0, #48]
        str     x13, [x0, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_alt_sub_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
