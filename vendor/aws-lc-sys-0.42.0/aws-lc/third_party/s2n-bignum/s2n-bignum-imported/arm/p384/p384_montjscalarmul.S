// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery-Jacobian form scalar multiplication for P-384
// Input scalar[6], point[18]; output res[18]
//
// extern void p384_montjscalarmul
//   (uint64_t res[static 18],
//    const uint64_t scalar[static 6],
//    const uint64_t point[static 18]);
//
// This function is a variant of its affine point version p384_scalarmul.
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

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjscalarmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjscalarmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjscalarmul)


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

S2N_BN_SYMBOL(p384_montjscalarmul):
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
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, tab+JACSIZE*2
        add     x1, tab+JACSIZE*1
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        add     x0, tab+JACSIZE*3
        add     x1, tab+JACSIZE*1
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, tab+JACSIZE*4
        add     x1, tab+JACSIZE*3
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        add     x0, tab+JACSIZE*5
        add     x1, tab+JACSIZE*2
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, tab+JACSIZE*6
        add     x1, tab+JACSIZE*5
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        add     x0, tab+JACSIZE*7
        add     x1, tab+JACSIZE*3
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, tab+JACSIZE*8
        add     x1, tab+JACSIZE*7
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        add     x0, tab+JACSIZE*9
        add     x1, tab+JACSIZE*4
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, tab+JACSIZE*10
        add     x1, tab+JACSIZE*9
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        add     x0, tab+JACSIZE*11
        add     x1, tab+JACSIZE*5
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, tab+JACSIZE*12
        add     x1, tab+JACSIZE*11
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        add     x0, tab+JACSIZE*13
        add     x1, tab+JACSIZE*6
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, tab+JACSIZE*14
        add     x1, tab+JACSIZE*13
        add     x2, tab
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        add     x0, tab+JACSIZE*15
        add     x1, tab+JACSIZE*7
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

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

Lp384_montjscalarmul_mainloop:
        sub     j, j, #5

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp384_montjscalarmul_p384_montjdouble)

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
        CFI_BL(Lp384_montjscalarmul_p384_montjadd)

        cbnz    j, Lp384_montjscalarmul_mainloop

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

S2N_BN_SIZE_DIRECTIVE(p384_montjscalarmul)

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp384_montjscalarmul_p384_montjadd)

Lp384_montjscalarmul_p384_montjadd:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH1Z(x27)
        CFI_DEC_SP(384)
        mov     x24, x0
        mov     x25, x1
        mov     x26, x2
        mov     x0, sp
        ldr     q1, [x25, #96]
        ldp     x9, x2, [x25, #96]
        ldr     q0, [x25, #96]
        ldp     x4, x6, [x25, #112]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [x25, #128]
        xtn     v30.2s, v0.2d
        ldr     q1, [x25, #128]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [x25, #128]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x15, x3, x17
        sbcs    x3, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [x0]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        stp     x15, x3, [x0, #16]
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [x0, #32]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        ldp     x9, x17, [x0, #16]
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [x0]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [x0, #32]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x9
        adcs    x1, x1, x17
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [x0]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [x0, #16]
        adc     x17, x14, xzr
        stp     x2, x17, [x0, #32]
        ldr     q1, [x26, #96]
        ldp     x9, x2, [x26, #96]
        ldr     q0, [x26, #96]
        ldp     x4, x6, [x26, #112]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [x26, #128]
        xtn     v30.2s, v0.2d
        ldr     q1, [x26, #128]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [x26, #128]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x15, x3, x17
        sbcs    x3, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [sp, #240]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        stp     x15, x3, [sp, #256]
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [sp, #272]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        ldp     x9, x17, [sp, #256]
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [sp, #240]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [sp, #272]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x9
        adcs    x1, x1, x17
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [sp, #240]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [sp, #256]
        adc     x17, x14, xzr
        stp     x2, x17, [sp, #272]
        stp     x23, x24, [sp, #0x150]
        ldr     q3, [x26, #96]
        ldr     q25, [x25, #48]
        ldp     x13, x23, [x25, #48]
        ldp     x3, x21, [x26, #96]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [x25, #80]
        ldp     x8, x24, [x26, #112]
        subs    x6, x3, x21
        ldr     q0, [x26, #128]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [x25, #64]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [x25, #80]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [x26, #128]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #288]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #304]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #320]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #288]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #304]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #320]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #288]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #304]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #320]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #288]
        ldp     x21, x12, [sp, #304]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #320]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #288]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #304]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #320]
        ldr     q3, [x25, #96]
        ldr     q25, [x26, #48]
        ldp     x13, x23, [x26, #48]
        ldp     x3, x21, [x25, #96]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [x26, #80]
        ldp     x8, x24, [x25, #112]
        subs    x6, x3, x21
        ldr     q0, [x25, #128]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [x26, #64]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [x26, #80]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [x25, #128]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #48]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #64]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #80]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #48]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #64]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #80]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #48]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #64]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #80]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #48]
        ldp     x21, x12, [sp, #64]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #80]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #48]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #64]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #80]
        mov     x1, sp
        ldr     q3, [x1]
        ldr     q25, [x26]
        ldp     x13, x23, [x26]
        ldp     x3, x21, [x1]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [x26, #32]
        ldp     x8, x24, [x1, #16]
        subs    x6, x3, x21
        ldr     q0, [x1, #32]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [x26, #16]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [x26, #32]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [x1, #32]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #96]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #112]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #128]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #96]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #112]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #128]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #96]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #112]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #128]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #96]
        ldp     x21, x12, [sp, #112]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #128]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #96]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #112]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #128]
        ldr     q3, [sp, #240]
        ldr     q25, [x25]
        ldp     x13, x23, [x25]
        ldp     x3, x21, [sp, #240]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [x25, #32]
        ldp     x8, x24, [sp, #256]
        subs    x6, x3, x21
        ldr     q0, [sp, #272]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [x25, #16]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [x25, #32]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #272]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #192]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #208]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #224]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #192]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #208]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #224]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #192]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #208]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #224]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #192]
        ldp     x21, x12, [sp, #208]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #224]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #192]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #208]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #224]
        mov     x1, sp
        ldr     q3, [x1]
        ldr     q25, [sp, #48]
        ldp     x13, x23, [sp, #48]
        ldp     x3, x21, [x1]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #80]
        ldp     x8, x24, [x1, #16]
        subs    x6, x3, x21
        ldr     q0, [x1, #32]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #64]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #80]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [x1, #32]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #48]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #64]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #80]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #48]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #64]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #80]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #48]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #64]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #80]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #48]
        ldp     x21, x12, [sp, #64]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #80]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #48]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #64]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #80]
        ldr     q3, [sp, #240]
        ldr     q25, [sp, #288]
        ldp     x13, x23, [sp, #288]
        ldp     x3, x21, [sp, #240]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #320]
        ldp     x8, x24, [sp, #256]
        subs    x6, x3, x21
        ldr     q0, [sp, #272]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #304]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #320]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #272]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #288]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #304]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #320]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #288]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #304]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #320]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #288]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #304]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #320]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #288]
        ldp     x21, x12, [sp, #304]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #320]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x2, x24, x11
        stp     x22, x5, [sp, #288]
        adcs    x11, x13, x23
        adcs    x12, x8, x23
        stp     x2, x11, [sp, #304]
        adc     x13, x15, x23
        stp     x12, x13, [sp, #320]
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
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
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
        sbcs    x7, x7, x2
        sbcs    x8, x8, x11
        ldp     x9, x10, [sp, #80]
        sbcs    x9, x9, x12
        sbcs    x10, x10, x13
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #48]
        stp     x7, x8, [sp, #64]
        stp     x9, x10, [sp, #80]
        ldr     q1, [sp, #240]
        ldp     x9, x2, [sp, #240]
        ldr     q0, [sp, #240]
        ldp     x4, x6, [sp, #256]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [sp, #272]
        xtn     v30.2s, v0.2d
        ldr     q1, [sp, #272]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [sp, #272]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x15, x3, x17
        sbcs    x3, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [sp, #144]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        stp     x15, x3, [sp, #160]
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [sp, #176]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        ldp     x9, x17, [sp, #160]
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [sp, #144]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [sp, #176]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x9
        adcs    x1, x1, x17
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [sp, #144]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [sp, #160]
        adc     x17, x14, xzr
        stp     x2, x17, [sp, #176]
        mov     x0, sp
        ldr     q1, [sp, #48]
        ldp     x9, x2, [sp, #48]
        ldr     q0, [sp, #48]
        ldp     x4, x6, [sp, #64]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [sp, #80]
        xtn     v30.2s, v0.2d
        ldr     q1, [sp, #80]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [sp, #80]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x15, x3, x17
        sbcs    x3, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [x0]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        stp     x15, x3, [x0, #16]
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [x0, #32]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        ldp     x9, x17, [x0, #16]
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [x0]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [x0, #32]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x9
        adcs    x1, x1, x17
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [x0]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [x0, #16]
        adc     x17, x14, xzr
        stp     x2, x17, [x0, #32]
        ldr     q3, [sp, #144]
        ldr     q25, [sp, #192]
        ldp     x13, x23, [sp, #192]
        ldp     x3, x21, [sp, #144]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #224]
        ldp     x8, x24, [sp, #160]
        subs    x6, x3, x21
        ldr     q0, [sp, #176]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #208]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #224]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #176]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #192]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #208]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #224]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #192]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #208]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #224]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #192]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #208]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #224]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #192]
        ldp     x21, x12, [sp, #208]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #224]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #192]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #208]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #224]
        ldr     q3, [sp, #144]
        ldr     q25, [sp, #96]
        ldp     x13, x23, [sp, #96]
        ldp     x3, x21, [sp, #144]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #128]
        ldp     x8, x24, [sp, #160]
        subs    x6, x3, x21
        ldr     q0, [sp, #176]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #112]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #128]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #176]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #96]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #112]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #128]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #96]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #112]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #128]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #96]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #112]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #128]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #96]
        ldp     x21, x12, [sp, #112]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #128]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x2, x24, x11
        stp     x22, x5, [sp, #96]
        adcs    x11, x13, x23
        adcs    x12, x8, x23
        stp     x2, x11, [sp, #112]
        adc     x13, x15, x23
        stp     x12, x13, [sp, #128]
        mov     x0, sp
        mov     x1, sp
        ldp     x5, x6, [x1]
        ldp     x4, x3, [sp, #192]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x1, #16]
        ldp     x4, x3, [sp, #208]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [x1, #32]
        ldp     x4, x3, [sp, #224]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [x0]
        stp     x7, x8, [x0, #16]
        stp     x9, x10, [x0, #32]
        ldp     x5, x6, [sp, #96]
        ldp     x4, x3, [sp, #192]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x4, x3, [sp, #208]
        sbcs    x7, x2, x4
        sbcs    x8, x11, x3
        ldp     x4, x3, [sp, #224]
        sbcs    x9, x12, x4
        sbcs    x10, x13, x3
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #144]
        stp     x7, x8, [sp, #160]
        stp     x9, x10, [sp, #176]
        ldr     q3, [sp, #240]
        ldr     q25, [x25, #96]
        ldp     x13, x23, [x25, #96]
        ldp     x3, x21, [sp, #240]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [x25, #128]
        ldp     x8, x24, [sp, #256]
        subs    x6, x3, x21
        ldr     q0, [sp, #272]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [x25, #112]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [x25, #128]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #272]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #240]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #256]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #272]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #240]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #256]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #272]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #240]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #256]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #272]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #240]
        ldp     x21, x12, [sp, #256]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #272]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #240]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #256]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #272]
        mov     x0, sp
        mov     x1, sp
        ldp     x5, x6, [x1]
        ldp     x4, x3, [sp, #96]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x1, #16]
        ldp     x4, x3, [sp, #112]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [x1, #32]
        ldp     x4, x3, [sp, #128]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x2, x5, x4
        eor     x4, x4, x3
        adcs    x11, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x4, x7, x4
        adcs    x12, x8, x3
        adcs    x13, x9, x3
        adc     x3, x10, x3
        stp     x2, x11, [x0]
        stp     x4, x12, [x0, #16]
        stp     x13, x3, [x0, #32]
        ldp     x5, x6, [sp, #192]
        subs    x5, x5, x2
        sbcs    x6, x6, x11
        ldp     x7, x8, [sp, #208]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x12
        ldp     x9, x10, [sp, #224]
        sbcs    x9, x9, x13
        sbcs    x10, x10, x3
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #192]
        stp     x7, x8, [sp, #208]
        stp     x9, x10, [sp, #224]
        ldr     q3, [sp, #144]
        ldr     q25, [sp, #288]
        ldp     x13, x23, [sp, #288]
        ldp     x3, x21, [sp, #144]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #320]
        ldp     x8, x24, [sp, #160]
        subs    x6, x3, x21
        ldr     q0, [sp, #176]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #304]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #320]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #176]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #144]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #160]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #176]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #144]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #160]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #176]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #144]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #160]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #176]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #144]
        ldp     x21, x12, [sp, #160]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #176]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #144]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #160]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #176]
        ldr     q3, [sp, #240]
        ldr     q25, [x26, #96]
        ldp     x13, x23, [x26, #96]
        ldp     x3, x21, [sp, #240]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [x26, #128]
        ldp     x8, x24, [sp, #256]
        subs    x6, x3, x21
        ldr     q0, [sp, #272]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [x26, #112]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [x26, #128]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #272]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #240]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #256]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #272]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #240]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #256]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #272]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #240]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #256]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #272]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #240]
        ldp     x21, x12, [sp, #256]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #272]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #240]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #256]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #272]
        ldp     x2, x27, [sp, #0x150]
        ldr     q3, [sp, #48]
        ldr     q25, [sp, #192]
        ldp     x13, x23, [sp, #192]
        ldp     x3, x21, [sp, #48]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #224]
        ldp     x8, x24, [sp, #64]
        subs    x6, x3, x21
        ldr     q0, [sp, #80]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #208]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #224]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #80]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #192]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #208]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #224]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #192]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #208]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #224]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #192]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #208]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #224]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #192]
        ldp     x21, x12, [sp, #208]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #224]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x2, x6, x20
        eor     x3, x20, x23
        adcs    x6, x7, x3
        adcs    x7, x24, x11
        adcs    x9, x13, x23
        adcs    x10, x8, x23
        adc     x11, x15, x23
        ldp     x4, x3, [sp, #144]
        subs    x5, x2, x4
        sbcs    x6, x6, x3
        ldp     x4, x3, [sp, #160]
        sbcs    x7, x7, x4
        sbcs    x8, x9, x3
        ldp     x4, x3, [sp, #176]
        sbcs    x9, x10, x4
        sbcs    x10, x11, x3
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x19, x5, x4
        eor     x4, x4, x3
        adcs    x24, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
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
        cset    x20, ne  // ne = any
        ldp     x6, x7, [x26, #96]
        ldp     x8, x9, [x26, #112]
        ldp     x10, x11, [x26, #128]
        orr     x21, x6, x7
        orr     x22, x8, x9
        orr     x23, x10, x11
        orr     x21, x21, x22
        orr     x21, x21, x23
        cmp     x21, xzr
        cset    x21, ne  // ne = any
        cmp     x21, x20
        ldp     x12, x13, [sp, #240]
        csel    x12, x0, x12, cc  // cc = lo, ul, last
        csel    x13, x1, x13, cc  // cc = lo, ul, last
        csel    x12, x6, x12, hi  // hi = pmore
        csel    x13, x7, x13, hi  // hi = pmore
        ldp     x14, x15, [sp, #256]
        csel    x14, x2, x14, cc  // cc = lo, ul, last
        csel    x15, x3, x15, cc  // cc = lo, ul, last
        csel    x14, x8, x14, hi  // hi = pmore
        csel    x15, x9, x15, hi  // hi = pmore
        ldp     x16, x17, [sp, #272]
        csel    x16, x4, x16, cc  // cc = lo, ul, last
        csel    x17, x5, x17, cc  // cc = lo, ul, last
        csel    x16, x10, x16, hi  // hi = pmore
        csel    x17, x11, x17, hi  // hi = pmore
        ldp     x20, x21, [x25]
        ldp     x0, x1, [sp]
        csel    x0, x20, x0, cc  // cc = lo, ul, last
        csel    x1, x21, x1, cc  // cc = lo, ul, last
        ldp     x20, x21, [x26]
        csel    x0, x20, x0, hi  // hi = pmore
        csel    x1, x21, x1, hi  // hi = pmore
        ldp     x20, x21, [x25, #16]
        ldp     x2, x3, [sp, #16]
        csel    x2, x20, x2, cc  // cc = lo, ul, last
        csel    x3, x21, x3, cc  // cc = lo, ul, last
        ldp     x20, x21, [x26, #16]
        csel    x2, x20, x2, hi  // hi = pmore
        csel    x3, x21, x3, hi  // hi = pmore
        ldp     x20, x21, [x25, #32]
        ldp     x4, x5, [sp, #32]
        csel    x4, x20, x4, cc  // cc = lo, ul, last
        csel    x5, x21, x5, cc  // cc = lo, ul, last
        ldp     x20, x21, [x26, #32]
        csel    x4, x20, x4, hi  // hi = pmore
        csel    x5, x21, x5, hi  // hi = pmore
        ldp     x20, x21, [x25, #48]
        csel    x6, x20, x19, cc  // cc = lo, ul, last
        csel    x7, x21, x24, cc  // cc = lo, ul, last
        ldp     x20, x21, [x26, #48]
        csel    x6, x20, x6, hi  // hi = pmore
        csel    x7, x21, x7, hi  // hi = pmore
        ldp     x20, x21, [x25, #64]
        ldp     x8, x9, [sp, #208]
        csel    x8, x20, x8, cc  // cc = lo, ul, last
        csel    x9, x21, x9, cc  // cc = lo, ul, last
        ldp     x20, x21, [x26, #64]
        csel    x8, x20, x8, hi  // hi = pmore
        csel    x9, x21, x9, hi  // hi = pmore
        ldp     x20, x21, [x25, #80]
        ldp     x10, x11, [sp, #224]
        csel    x10, x20, x10, cc  // cc = lo, ul, last
        csel    x11, x21, x11, cc  // cc = lo, ul, last
        ldp     x20, x21, [x26, #80]
        csel    x10, x20, x10, hi  // hi = pmore
        csel    x11, x21, x11, hi  // hi = pmore
        stp     x0, x1, [x27]
        stp     x2, x3, [x27, #16]
        stp     x4, x5, [x27, #32]
        stp     x6, x7, [x27, #48]
        stp     x8, x9, [x27, #64]
        stp     x10, x11, [x27, #80]
        stp     x12, x13, [x27, #96]
        stp     x14, x15, [x27, #112]
        stp     x16, x17, [x27, #128]
        CFI_INC_SP(384)
        CFI_POP1Z(x27)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp384_montjscalarmul_p384_montjadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp384_montjscalarmul_p384_montjdouble)

Lp384_montjscalarmul_p384_montjdouble:
        CFI_START
        CFI_DEC_SP(416)
        stp     x19, x20, [sp, #336]
        stp     x21, x22, [sp, #352]
        stp     x23, x24, [sp, #368]
        stp     x25, x26, [sp, #384]
        stp     x27, xzr, [sp, #400]
        mov     x25, x0
        mov     x26, x1
        mov     x0, sp
        ldr     q1, [x26, #96]
        ldp     x9, x2, [x26, #96]
        ldr     q0, [x26, #96]
        ldp     x4, x6, [x26, #112]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [x26, #128]
        xtn     v30.2s, v0.2d
        ldr     q1, [x26, #128]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [x26, #128]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x15, x3, x17
        sbcs    x3, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [x0]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        stp     x15, x3, [x0, #16]
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [x0, #32]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        ldp     x9, x17, [x0, #16]
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [x0]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [x0, #32]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x9
        adcs    x1, x1, x17
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [x0]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [x0, #16]
        adc     x17, x14, xzr
        stp     x2, x17, [x0, #32]
        ldr     q1, [x26, #48]
        ldp     x9, x2, [x26, #48]
        ldr     q0, [x26, #48]
        ldp     x4, x6, [x26, #64]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [x26, #80]
        xtn     v30.2s, v0.2d
        ldr     q1, [x26, #80]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [x26, #80]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x15, x3, x17
        sbcs    x3, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [sp, #48]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        stp     x15, x3, [sp, #64]
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [sp, #80]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        ldp     x9, x17, [sp, #64]
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [sp, #48]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [sp, #80]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x9
        adcs    x1, x1, x17
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [sp, #48]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [sp, #64]
        adc     x17, x14, xzr
        stp     x2, x17, [sp, #80]
        ldp     x5, x6, [x26]
        ldp     x4, x3, [sp]
        adds    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x26, #16]
        ldp     x4, x3, [sp, #16]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        ldp     x9, x10, [x26, #32]
        ldp     x4, x3, [sp, #32]
        adcs    x9, x9, x4
        adcs    x10, x10, x3
        csetm   x3, cs  // cs = hs, nlast
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        subs    x5, x5, x4
        eor     x4, x4, x3
        sbcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        sbcs    x9, x9, x3
        sbc     x10, x10, x3
        stp     x5, x6, [sp, #240]
        stp     x7, x8, [sp, #256]
        stp     x9, x10, [sp, #272]
        mov     x2, sp
        ldp     x5, x6, [x26]
        ldp     x4, x3, [x2]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x26, #16]
        ldp     x4, x3, [x2, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [x26, #32]
        ldp     x4, x3, [x2, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x13, x5, x4
        eor     x4, x4, x3
        adcs    x23, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x13, x23, [sp, #192]
        stp     x7, x8, [sp, #208]
        stp     x9, x10, [sp, #224]
        ldr     q3, [sp, #240]
        ldr     q25, [sp, #192]
        ldp     x3, x21, [sp, #240]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #224]
        ldp     x8, x24, [sp, #256]
        subs    x6, x3, x21
        ldr     q0, [sp, #272]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #208]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #224]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #272]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x16, x4, x16
        mov     x4, v27.d[0]
        sbcs    x11, x20, x11
        sbcs    x20, x9, x12
        stp     x16, x11, [sp, #96]
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #112]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #128]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        ldp     x20, x9, [sp, #96]
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #112]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #128]
        adds    x20, x22, x20
        mul     x10, x13, x14
        adcs    x11, x11, x9
        eor     x9, x8, x21
        adcs    x21, x19, x17
        stp     x20, x11, [sp, #96]
        adcs    x12, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        stp     x21, x12, [sp, #112]
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #128]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #96]
        ldp     x21, x12, [sp, #112]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #128]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x21
        eor     x1, x22, x9
        adcs    x24, x23, x12
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x21
        adcs    x15, x17, x12
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #96]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #112]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #128]
        ldp     x5, x6, [x26, #48]
        ldp     x4, x3, [x26, #96]
        adds    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x26, #64]
        ldp     x4, x3, [x26, #112]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        ldp     x9, x10, [x26, #80]
        ldp     x4, x3, [x26, #128]
        adcs    x9, x9, x4
        adcs    x10, x10, x3
        adc     x3, xzr, xzr
        mov     x4, #0xffffffff                 // #4294967295
        cmp     x5, x4
        mov     x4, #0xffffffff00000000         // #-4294967296
        sbcs    xzr, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        sbcs    xzr, x7, x4
        adcs    xzr, x8, xzr
        adcs    xzr, x9, xzr
        adcs    xzr, x10, xzr
        adcs    x3, x3, xzr
        csetm   x3, ne  // ne = any
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        subs    x5, x5, x4
        eor     x4, x4, x3
        sbcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        sbcs    x9, x9, x3
        sbc     x10, x10, x3
        stp     x5, x6, [sp, #240]
        stp     x7, x8, [sp, #256]
        stp     x9, x10, [sp, #272]
        ldr     q1, [sp, #96]
        ldp     x9, x2, [sp, #96]
        ldr     q0, [sp, #96]
        ldp     x4, x6, [sp, #112]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [sp, #128]
        xtn     v30.2s, v0.2d
        ldr     q1, [sp, #128]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [sp, #128]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x15, x3, x17
        sbcs    x3, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [sp, #288]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        stp     x15, x3, [sp, #304]
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [sp, #320]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        ldp     x9, x17, [sp, #304]
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [sp, #288]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [sp, #320]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x9
        adcs    x1, x1, x17
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [sp, #288]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [sp, #304]
        adc     x17, x14, xzr
        stp     x2, x17, [sp, #320]
        ldr     q3, [x26]
        ldr     q25, [sp, #48]
        ldp     x13, x23, [sp, #48]
        ldp     x3, x21, [x26]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #80]
        ldp     x8, x24, [x26, #16]
        subs    x6, x3, x21
        ldr     q0, [x26, #32]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #64]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #80]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [x26, #32]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x26, x4, x16
        mov     x4, v27.d[0]
        sbcs    x27, x20, x11
        sbcs    x20, x9, x12
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #160]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #176]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #160]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #176]
        adds    x20, x22, x26
        mul     x10, x13, x14
        adcs    x11, x11, x27
        eor     x9, x8, x21
        adcs    x26, x19, x17
        stp     x20, x11, [sp, #144]
        adcs    x27, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #176]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #144]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #176]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x26
        eor     x1, x22, x9
        adcs    x24, x23, x27
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x26
        adcs    x15, x17, x27
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #144]
        adcs    x5, x13, x23
        adcs    x21, x8, x23
        stp     x14, x5, [sp, #160]
        adc     x12, x15, x23
        stp     x21, x12, [sp, #176]
        ldr     q1, [sp, #240]
        ldp     x9, x2, [sp, #240]
        ldr     q0, [sp, #240]
        ldp     x4, x6, [sp, #256]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [sp, #272]
        xtn     v30.2s, v0.2d
        ldr     q1, [sp, #272]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [sp, #272]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x19, x3, x17
        sbcs    x20, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [sp, #192]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [sp, #224]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [sp, #192]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [sp, #224]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x19
        adcs    x1, x1, x20
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x19, x13, x1
        and     x13, x4, x9
        adcs    x20, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [sp, #208]
        adc     x17, x14, xzr
        stp     x2, x17, [sp, #224]
        ldp     x0, x1, [sp, #288]
        mov     x6, #0xffffffff                 // #4294967295
        subs    x6, x6, x0
        mov     x7, #0xffffffff00000000         // #-4294967296
        sbcs    x7, x7, x1
        ldp     x0, x1, [sp, #304]
        mov     x8, #0xfffffffffffffffe         // #-2
        sbcs    x8, x8, x0
        mov     x13, #0xffffffffffffffff        // #-1
        sbcs    x9, x13, x1
        ldp     x0, x1, [sp, #320]
        sbcs    x10, x13, x0
        sbc     x11, x13, x1
        mov     x12, #0x9                       // #9
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
        mov     x6, #0x1                        // #1
        adc     x6, x12, x6
        ldp     x8, x9, [sp, #144]
        ldp     x10, x11, [sp, #160]
        ldp     x12, x13, [sp, #176]
        mov     x14, #0xc                       // #12
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
        csetm   x6, cc  // cc = lo, ul, last
        mov     x7, #0xffffffff                 // #4294967295
        and     x7, x7, x6
        adds    x0, x0, x7
        eor     x7, x7, x6
        adcs    x1, x1, x7
        mov     x7, #0xfffffffffffffffe         // #-2
        and     x7, x7, x6
        adcs    x2, x2, x7
        adcs    x3, x3, x6
        adcs    x4, x4, x6
        adc     x5, x5, x6
        stp     x0, x1, [sp, #288]
        stp     x2, x3, [sp, #304]
        stp     x4, x5, [sp, #320]
        mov     x2, sp
        ldp     x4, x3, [x2]
        subs    x5, x19, x4
        sbcs    x6, x20, x3
        ldp     x7, x8, [sp, #208]
        ldp     x4, x3, [x2, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        ldp     x9, x10, [sp, #224]
        ldp     x4, x3, [x2, #32]
        sbcs    x9, x9, x4
        sbcs    x10, x10, x3
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [sp, #240]
        stp     x7, x8, [sp, #256]
        stp     x9, x10, [sp, #272]
        ldr     q1, [sp, #48]
        ldp     x9, x2, [sp, #48]
        ldr     q0, [sp, #48]
        ldp     x4, x6, [sp, #64]
        rev64   v21.4s, v1.4s
        uzp2    v28.4s, v1.4s, v1.4s
        umulh   x7, x9, x2
        xtn     v17.2s, v1.2d
        mul     v27.4s, v21.4s, v0.4s
        ldr     q20, [sp, #80]
        xtn     v30.2s, v0.2d
        ldr     q1, [sp, #80]
        uzp2    v31.4s, v0.4s, v0.4s
        ldp     x5, x10, [sp, #80]
        umulh   x8, x9, x4
        uaddlp  v3.2d, v27.4s
        umull   v16.2d, v30.2s, v17.2s
        mul     x16, x9, x4
        umull   v27.2d, v30.2s, v28.2s
        shrn    v0.2s, v20.2d, #32
        xtn     v7.2s, v20.2d
        shl     v20.2d, v3.2d, #32
        umull   v3.2d, v31.2s, v28.2s
        mul     x3, x2, x4
        umlal   v20.2d, v30.2s, v17.2s
        umull   v22.2d, v7.2s, v0.2s
        usra    v27.2d, v16.2d, #32
        umulh   x11, x2, x4
        movi    v21.2d, #0xffffffff
        uzp2    v28.4s, v1.4s, v1.4s
        adds    x15, x16, x7
        and     v5.16b, v27.16b, v21.16b
        adcs    x3, x3, x8
        usra    v3.2d, v27.2d, #32
        dup     v29.2d, x6
        adcs    x16, x11, xzr
        mov     x14, v20.d[0]
        umlal   v5.2d, v31.2s, v17.2s
        mul     x8, x9, x2
        mov     x7, v20.d[1]
        shl     v19.2d, v22.2d, #33
        xtn     v25.2s, v29.2d
        rev64   v31.4s, v1.4s
        lsl     x13, x14, #32
        uzp2    v6.4s, v29.4s, v29.4s
        umlal   v19.2d, v7.2s, v7.2s
        usra    v3.2d, v5.2d, #32
        adds    x1, x8, x8
        umulh   x8, x4, x4
        add     x12, x13, x14
        mul     v17.4s, v31.4s, v29.4s
        xtn     v4.2s, v1.2d
        adcs    x14, x15, x15
        lsr     x13, x12, #32
        adcs    x15, x3, x3
        umull   v31.2d, v25.2s, v28.2s
        adcs    x11, x16, x16
        umull   v21.2d, v25.2s, v4.2s
        mov     x17, v3.d[0]
        umull   v18.2d, v6.2s, v28.2s
        adc     x16, x8, xzr
        uaddlp  v16.2d, v17.4s
        movi    v1.2d, #0xffffffff
        subs    x13, x13, x12
        usra    v31.2d, v21.2d, #32
        sbc     x8, x12, xzr
        adds    x17, x17, x1
        mul     x1, x4, x4
        shl     v28.2d, v16.2d, #32
        mov     x3, v3.d[1]
        adcs    x14, x7, x14
        extr    x7, x8, x13, #32
        adcs    x13, x3, x15
        and     v3.16b, v31.16b, v1.16b
        adcs    x11, x1, x11
        lsr     x1, x8, #32
        umlal   v3.2d, v6.2s, v4.2s
        usra    v18.2d, v31.2d, #32
        adc     x3, x16, xzr
        adds    x1, x1, x12
        umlal   v28.2d, v25.2s, v4.2s
        adc     x16, xzr, xzr
        subs    x15, x17, x7
        sbcs    x7, x14, x1
        lsl     x1, x15, #32
        sbcs    x16, x13, x16
        add     x8, x1, x15
        usra    v18.2d, v3.2d, #32
        sbcs    x14, x11, xzr
        lsr     x1, x8, #32
        sbcs    x17, x3, xzr
        sbc     x11, x12, xzr
        subs    x13, x1, x8
        umulh   x12, x4, x10
        sbc     x1, x8, xzr
        extr    x13, x1, x13, #32
        lsr     x1, x1, #32
        adds    x15, x1, x8
        adc     x1, xzr, xzr
        subs    x7, x7, x13
        sbcs    x13, x16, x15
        lsl     x3, x7, #32
        umulh   x16, x2, x5
        sbcs    x15, x14, x1
        add     x7, x3, x7
        sbcs    x3, x17, xzr
        lsr     x1, x7, #32
        sbcs    x14, x11, xzr
        sbc     x11, x8, xzr
        subs    x8, x1, x7
        sbc     x1, x7, xzr
        extr    x8, x1, x8, #32
        lsr     x1, x1, #32
        adds    x1, x1, x7
        adc     x17, xzr, xzr
        subs    x13, x13, x8
        umulh   x8, x9, x6
        sbcs    x1, x15, x1
        sbcs    x19, x3, x17
        sbcs    x20, x14, xzr
        mul     x17, x2, x5
        sbcs    x11, x11, xzr
        stp     x13, x1, [sp, #192]
        sbc     x14, x7, xzr
        mul     x7, x4, x10
        subs    x1, x9, x2
        csetm   x15, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        stp     x11, x14, [sp, #224]
        mul     x14, x9, x6
        adds    x17, x8, x17
        adcs    x7, x16, x7
        adc     x13, x12, xzr
        subs    x12, x5, x6
        cneg    x3, x12, cc  // cc = lo, ul, last
        cinv    x16, x15, cc  // cc = lo, ul, last
        mul     x8, x1, x3
        umulh   x1, x1, x3
        eor     x12, x8, x16
        adds    x11, x17, x14
        adcs    x3, x7, x17
        adcs    x15, x13, x7
        adc     x8, x13, xzr
        adds    x3, x3, x14
        adcs    x15, x15, x17
        adcs    x17, x8, x7
        eor     x1, x1, x16
        adc     x13, x13, xzr
        subs    x9, x9, x4
        csetm   x8, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x4, x2, x4
        cneg    x4, x4, cc  // cc = lo, ul, last
        csetm   x7, cc  // cc = lo, ul, last
        subs    x2, x10, x6
        cinv    x8, x8, cc  // cc = lo, ul, last
        cneg    x2, x2, cc  // cc = lo, ul, last
        cmn     x16, #0x1
        adcs    x11, x11, x12
        mul     x12, x9, x2
        adcs    x3, x3, x1
        adcs    x15, x15, x16
        umulh   x9, x9, x2
        adcs    x17, x17, x16
        adc     x13, x13, x16
        subs    x1, x10, x5
        cinv    x2, x7, cc  // cc = lo, ul, last
        cneg    x1, x1, cc  // cc = lo, ul, last
        eor     x9, x9, x8
        cmn     x8, #0x1
        eor     x7, x12, x8
        mul     x12, x4, x1
        adcs    x3, x3, x7
        adcs    x7, x15, x9
        adcs    x15, x17, x8
        umulh   x4, x4, x1
        adc     x8, x13, x8
        cmn     x2, #0x1
        eor     x1, x12, x2
        adcs    x1, x7, x1
        ldp     x7, x16, [sp, #192]
        eor     x12, x4, x2
        adcs    x4, x15, x12
        ldp     x15, x12, [sp, #224]
        adc     x8, x8, x2
        adds    x13, x14, x14
        umulh   x14, x5, x10
        adcs    x2, x11, x11
        adcs    x3, x3, x3
        adcs    x1, x1, x1
        adcs    x4, x4, x4
        adcs    x11, x8, x8
        adc     x8, xzr, xzr
        adds    x13, x13, x7
        adcs    x2, x2, x16
        mul     x16, x5, x10
        adcs    x3, x3, x19
        adcs    x1, x1, x20
        umulh   x5, x5, x5
        lsl     x9, x13, #32
        add     x9, x9, x13
        adcs    x4, x4, x15
        mov     x13, v28.d[1]
        adcs    x15, x11, x12
        lsr     x7, x9, #32
        adc     x11, x8, xzr
        subs    x7, x7, x9
        umulh   x10, x10, x10
        sbc     x17, x9, xzr
        extr    x7, x17, x7, #32
        lsr     x17, x17, #32
        adds    x17, x17, x9
        adc     x12, xzr, xzr
        subs    x8, x2, x7
        sbcs    x17, x3, x17
        lsl     x7, x8, #32
        sbcs    x2, x1, x12
        add     x3, x7, x8
        sbcs    x12, x4, xzr
        lsr     x1, x3, #32
        sbcs    x7, x15, xzr
        sbc     x15, x9, xzr
        subs    x1, x1, x3
        sbc     x4, x3, xzr
        lsr     x9, x4, #32
        extr    x8, x4, x1, #32
        adds    x9, x9, x3
        adc     x4, xzr, xzr
        subs    x1, x17, x8
        lsl     x17, x1, #32
        sbcs    x8, x2, x9
        sbcs    x9, x12, x4
        add     x17, x17, x1
        mov     x1, v18.d[1]
        lsr     x2, x17, #32
        sbcs    x7, x7, xzr
        mov     x12, v18.d[0]
        sbcs    x15, x15, xzr
        sbc     x3, x3, xzr
        subs    x4, x2, x17
        sbc     x2, x17, xzr
        adds    x12, x13, x12
        adcs    x16, x16, x1
        lsr     x13, x2, #32
        extr    x1, x2, x4, #32
        adc     x2, x14, xzr
        adds    x4, x13, x17
        mul     x13, x6, x6
        adc     x14, xzr, xzr
        subs    x1, x8, x1
        sbcs    x4, x9, x4
        mov     x9, v28.d[0]
        sbcs    x7, x7, x14
        sbcs    x8, x15, xzr
        sbcs    x3, x3, xzr
        sbc     x14, x17, xzr
        adds    x17, x9, x9
        adcs    x12, x12, x12
        mov     x15, v19.d[0]
        adcs    x9, x16, x16
        umulh   x6, x6, x6
        adcs    x16, x2, x2
        adc     x2, xzr, xzr
        adds    x11, x11, x8
        adcs    x3, x3, xzr
        adcs    x14, x14, xzr
        adcs    x8, xzr, xzr
        adds    x13, x1, x13
        mov     x1, v19.d[1]
        adcs    x6, x4, x6
        mov     x4, #0xffffffff                 // #4294967295
        adcs    x15, x7, x15
        adcs    x7, x11, x5
        adcs    x1, x3, x1
        adcs    x14, x14, x10
        adc     x11, x8, xzr
        adds    x6, x6, x17
        adcs    x8, x15, x12
        adcs    x3, x7, x9
        adcs    x15, x1, x16
        mov     x16, #0xffffffff00000001        // #-4294967295
        adcs    x14, x14, x2
        mov     x2, #0x1                        // #1
        adc     x17, x11, xzr
        cmn     x13, x16
        adcs    xzr, x6, x4
        adcs    xzr, x8, x2
        adcs    xzr, x3, xzr
        adcs    xzr, x15, xzr
        adcs    xzr, x14, xzr
        adc     x1, x17, xzr
        neg     x9, x1
        and     x1, x16, x9
        adds    x11, x13, x1
        and     x13, x4, x9
        adcs    x5, x6, x13
        and     x1, x2, x9
        adcs    x7, x8, x1
        stp     x11, x5, [sp, #192]
        adcs    x11, x3, xzr
        adcs    x2, x15, xzr
        stp     x7, x11, [sp, #208]
        adc     x17, x14, xzr
        stp     x2, x17, [sp, #224]
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
        csetm   x3, cc  // cc = lo, ul, last
        mov     x4, #0xffffffff                 // #4294967295
        and     x4, x4, x3
        adds    x5, x5, x4
        eor     x4, x4, x3
        adcs    x6, x6, x4
        mov     x4, #0xfffffffffffffffe         // #-2
        and     x4, x4, x3
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adcs    x9, x9, x3
        adc     x10, x10, x3
        stp     x5, x6, [x25, #96]
        stp     x7, x8, [x25, #112]
        stp     x9, x10, [x25, #128]
        ldr     q3, [sp, #288]
        ldr     q25, [sp, #96]
        ldp     x13, x23, [sp, #96]
        ldp     x3, x21, [sp, #288]
        rev64   v23.4s, v25.4s
        uzp1    v17.4s, v25.4s, v3.4s
        umulh   x15, x3, x13
        mul     v6.4s, v23.4s, v3.4s
        uzp1    v3.4s, v3.4s, v3.4s
        ldr     q27, [sp, #128]
        ldp     x8, x24, [sp, #304]
        subs    x6, x3, x21
        ldr     q0, [sp, #320]
        movi    v23.2d, #0xffffffff
        csetm   x10, cc  // cc = lo, ul, last
        umulh   x19, x21, x23
        rev64   v4.4s, v27.4s
        uzp2    v25.4s, v27.4s, v27.4s
        cneg    x4, x6, cc  // cc = lo, ul, last
        subs    x7, x23, x13
        xtn     v22.2s, v0.2d
        xtn     v24.2s, v27.2d
        cneg    x20, x7, cc  // cc = lo, ul, last
        ldp     x6, x14, [sp, #112]
        mul     v27.4s, v4.4s, v0.4s
        uaddlp  v20.2d, v6.4s
        cinv    x5, x10, cc  // cc = lo, ul, last
        mul     x16, x4, x20
        uzp2    v6.4s, v0.4s, v0.4s
        umull   v21.2d, v22.2s, v25.2s
        shl     v0.2d, v20.2d, #32
        umlal   v0.2d, v3.2s, v17.2s
        mul     x22, x8, x6
        umull   v1.2d, v6.2s, v25.2s
        subs    x12, x3, x8
        umull   v20.2d, v22.2s, v24.2s
        cneg    x17, x12, cc  // cc = lo, ul, last
        umulh   x9, x8, x6
        mov     x12, v0.d[1]
        eor     x11, x16, x5
        mov     x7, v0.d[0]
        csetm   x10, cc  // cc = lo, ul, last
        usra    v21.2d, v20.2d, #32
        adds    x15, x15, x12
        adcs    x12, x19, x22
        umulh   x20, x4, x20
        adc     x19, x9, xzr
        usra    v1.2d, v21.2d, #32
        adds    x22, x15, x7
        and     v26.16b, v21.16b, v23.16b
        adcs    x16, x12, x15
        uaddlp  v25.2d, v27.4s
        adcs    x9, x19, x12
        umlal   v26.2d, v6.2s, v24.2s
        adc     x4, x19, xzr
        adds    x16, x16, x7
        shl     v27.2d, v25.2d, #32
        adcs    x9, x9, x15
        adcs    x4, x4, x12
        eor     x12, x20, x5
        adc     x15, x19, xzr
        subs    x20, x6, x13
        cneg    x20, x20, cc  // cc = lo, ul, last
        cinv    x10, x10, cc  // cc = lo, ul, last
        cmn     x5, #0x1
        mul     x19, x17, x20
        adcs    x11, x22, x11
        adcs    x12, x16, x12
        adcs    x9, x9, x5
        umulh   x17, x17, x20
        adcs    x22, x4, x5
        adc     x5, x15, x5
        subs    x16, x21, x8
        cneg    x20, x16, cc  // cc = lo, ul, last
        eor     x19, x19, x10
        csetm   x4, cc  // cc = lo, ul, last
        subs    x16, x6, x23
        cneg    x16, x16, cc  // cc = lo, ul, last
        umlal   v27.2d, v22.2s, v24.2s
        mul     x15, x20, x16
        cinv    x4, x4, cc  // cc = lo, ul, last
        cmn     x10, #0x1
        usra    v1.2d, v26.2d, #32
        adcs    x19, x12, x19
        eor     x17, x17, x10
        adcs    x9, x9, x17
        adcs    x22, x22, x10
        lsl     x12, x7, #32
        umulh   x20, x20, x16
        eor     x16, x15, x4
        ldp     x15, x17, [sp, #128]
        add     x2, x12, x7
        adc     x7, x5, x10
        ldp     x5, x10, [sp, #320]
        lsr     x1, x2, #32
        eor     x12, x20, x4
        subs    x1, x1, x2
        sbc     x20, x2, xzr
        cmn     x4, #0x1
        adcs    x9, x9, x16
        extr    x1, x20, x1, #32
        lsr     x20, x20, #32
        adcs    x22, x22, x12
        adc     x16, x7, x4
        adds    x12, x20, x2
        umulh   x7, x24, x14
        adc     x4, xzr, xzr
        subs    x1, x11, x1
        sbcs    x20, x19, x12
        sbcs    x12, x9, x4
        lsl     x9, x1, #32
        add     x1, x9, x1
        sbcs    x9, x22, xzr
        mul     x22, x24, x14
        sbcs    x16, x16, xzr
        lsr     x4, x1, #32
        sbc     x19, x2, xzr
        subs    x4, x4, x1
        sbc     x11, x1, xzr
        extr    x2, x11, x4, #32
        lsr     x4, x11, #32
        adds    x4, x4, x1
        adc     x11, xzr, xzr
        subs    x2, x20, x2
        sbcs    x4, x12, x4
        sbcs    x20, x9, x11
        lsl     x12, x2, #32
        add     x2, x12, x2
        sbcs    x9, x16, xzr
        lsr     x11, x2, #32
        sbcs    x19, x19, xzr
        sbc     x1, x1, xzr
        subs    x16, x11, x2
        sbc     x12, x2, xzr
        extr    x16, x12, x16, #32
        lsr     x12, x12, #32
        adds    x11, x12, x2
        adc     x12, xzr, xzr
        subs    x26, x4, x16
        mov     x4, v27.d[0]
        sbcs    x27, x20, x11
        sbcs    x20, x9, x12
        sbcs    x11, x19, xzr
        sbcs    x9, x1, xzr
        stp     x20, x11, [sp, #256]
        mov     x1, v1.d[0]
        sbc     x20, x2, xzr
        subs    x12, x24, x5
        mov     x11, v27.d[1]
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x2, cc  // cc = lo, ul, last
        subs    x19, x15, x14
        mov     x12, v1.d[1]
        cinv    x2, x2, cc  // cc = lo, ul, last
        cneg    x19, x19, cc  // cc = lo, ul, last
        stp     x9, x20, [sp, #272]
        mul     x9, x16, x19
        adds    x4, x7, x4
        adcs    x11, x1, x11
        adc     x1, x12, xzr
        adds    x20, x4, x22
        umulh   x19, x16, x19
        adcs    x7, x11, x4
        eor     x16, x9, x2
        adcs    x9, x1, x11
        adc     x12, x1, xzr
        adds    x7, x7, x22
        adcs    x4, x9, x4
        adcs    x9, x12, x11
        adc     x12, x1, xzr
        cmn     x2, #0x1
        eor     x1, x19, x2
        adcs    x11, x20, x16
        adcs    x19, x7, x1
        adcs    x1, x4, x2
        adcs    x20, x9, x2
        adc     x2, x12, x2
        subs    x12, x24, x10
        cneg    x16, x12, cc  // cc = lo, ul, last
        csetm   x12, cc  // cc = lo, ul, last
        subs    x9, x17, x14
        cinv    x12, x12, cc  // cc = lo, ul, last
        cneg    x9, x9, cc  // cc = lo, ul, last
        subs    x3, x24, x3
        sbcs    x21, x5, x21
        mul     x24, x16, x9
        sbcs    x4, x10, x8
        ngc     x8, xzr
        subs    x10, x5, x10
        eor     x5, x24, x12
        csetm   x7, cc  // cc = lo, ul, last
        cneg    x24, x10, cc  // cc = lo, ul, last
        subs    x10, x17, x15
        cinv    x7, x7, cc  // cc = lo, ul, last
        cneg    x10, x10, cc  // cc = lo, ul, last
        subs    x14, x13, x14
        sbcs    x15, x23, x15
        eor     x13, x21, x8
        mul     x23, x24, x10
        sbcs    x17, x6, x17
        eor     x6, x3, x8
        ngc     x21, xzr
        umulh   x9, x16, x9
        cmn     x8, #0x1
        eor     x3, x23, x7
        adcs    x23, x6, xzr
        adcs    x13, x13, xzr
        eor     x16, x4, x8
        adc     x16, x16, xzr
        eor     x4, x17, x21
        umulh   x17, x24, x10
        cmn     x21, #0x1
        eor     x24, x14, x21
        eor     x6, x15, x21
        adcs    x15, x24, xzr
        adcs    x14, x6, xzr
        adc     x6, x4, xzr
        cmn     x12, #0x1
        eor     x4, x9, x12
        adcs    x19, x19, x5
        umulh   x5, x23, x15
        adcs    x1, x1, x4
        adcs    x10, x20, x12
        eor     x4, x17, x7
        adc     x2, x2, x12
        cmn     x7, #0x1
        adcs    x12, x1, x3
        ldp     x17, x24, [sp, #256]
        mul     x1, x16, x6
        adcs    x3, x10, x4
        adc     x2, x2, x7
        ldp     x7, x4, [sp, #272]
        adds    x20, x22, x26
        mul     x10, x13, x14
        adcs    x11, x11, x27
        eor     x9, x8, x21
        adcs    x26, x19, x17
        stp     x20, x11, [sp, #240]
        adcs    x27, x12, x24
        mul     x8, x23, x15
        adcs    x3, x3, x7
        adcs    x12, x2, x4
        adc     x19, xzr, xzr
        subs    x21, x23, x16
        umulh   x2, x16, x6
        stp     x3, x12, [sp, #272]
        cneg    x3, x21, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        umulh   x11, x13, x14
        subs    x21, x13, x16
        eor     x7, x8, x9
        cneg    x17, x21, cc  // cc = lo, ul, last
        csetm   x16, cc  // cc = lo, ul, last
        subs    x21, x6, x15
        cneg    x22, x21, cc  // cc = lo, ul, last
        cinv    x21, x24, cc  // cc = lo, ul, last
        subs    x20, x23, x13
        umulh   x12, x3, x22
        cneg    x23, x20, cc  // cc = lo, ul, last
        csetm   x24, cc  // cc = lo, ul, last
        subs    x20, x14, x15
        cinv    x24, x24, cc  // cc = lo, ul, last
        mul     x22, x3, x22
        cneg    x3, x20, cc  // cc = lo, ul, last
        subs    x13, x6, x14
        cneg    x20, x13, cc  // cc = lo, ul, last
        cinv    x15, x16, cc  // cc = lo, ul, last
        adds    x13, x5, x10
        mul     x4, x23, x3
        adcs    x11, x11, x1
        adc     x14, x2, xzr
        adds    x5, x13, x8
        adcs    x16, x11, x13
        umulh   x23, x23, x3
        adcs    x3, x14, x11
        adc     x1, x14, xzr
        adds    x10, x16, x8
        adcs    x6, x3, x13
        adcs    x8, x1, x11
        umulh   x13, x17, x20
        eor     x1, x4, x24
        adc     x4, x14, xzr
        cmn     x24, #0x1
        adcs    x1, x5, x1
        eor     x16, x23, x24
        eor     x11, x1, x9
        adcs    x23, x10, x16
        eor     x2, x22, x21
        adcs    x3, x6, x24
        mul     x14, x17, x20
        eor     x17, x13, x15
        adcs    x13, x8, x24
        adc     x8, x4, x24
        cmn     x21, #0x1
        adcs    x6, x23, x2
        mov     x16, #0xfffffffffffffffe        // #-2
        eor     x20, x12, x21
        adcs    x20, x3, x20
        eor     x23, x14, x15
        adcs    x2, x13, x21
        adc     x8, x8, x21
        cmn     x15, #0x1
        ldp     x5, x4, [sp, #240]
        adcs    x22, x20, x23
        eor     x23, x22, x9
        adcs    x17, x2, x17
        adc     x22, x8, x15
        cmn     x9, #0x1
        adcs    x15, x7, x5
        ldp     x10, x14, [sp, #272]
        eor     x1, x6, x9
        lsl     x2, x15, #32
        adcs    x8, x11, x4
        adcs    x13, x1, x26
        eor     x1, x22, x9
        adcs    x24, x23, x27
        eor     x11, x17, x9
        adcs    x23, x11, x10
        adcs    x7, x1, x14
        adcs    x17, x9, x19
        adcs    x20, x9, xzr
        add     x1, x2, x15
        lsr     x3, x1, #32
        adcs    x11, x9, xzr
        adc     x9, x9, xzr
        subs    x3, x3, x1
        sbc     x6, x1, xzr
        adds    x24, x24, x5
        adcs    x4, x23, x4
        extr    x3, x6, x3, #32
        lsr     x6, x6, #32
        adcs    x21, x7, x26
        adcs    x15, x17, x27
        adcs    x7, x20, x10
        adcs    x20, x11, x14
        mov     x14, #0xffffffff                // #4294967295
        adc     x22, x9, x19
        adds    x12, x6, x1
        adc     x10, xzr, xzr
        subs    x3, x8, x3
        sbcs    x12, x13, x12
        lsl     x9, x3, #32
        add     x3, x9, x3
        sbcs    x10, x24, x10
        sbcs    x24, x4, xzr
        lsr     x9, x3, #32
        sbcs    x21, x21, xzr
        sbc     x1, x1, xzr
        subs    x9, x9, x3
        sbc     x13, x3, xzr
        extr    x9, x13, x9, #32
        lsr     x13, x13, #32
        adds    x13, x13, x3
        adc     x6, xzr, xzr
        subs    x12, x12, x9
        sbcs    x17, x10, x13
        lsl     x2, x12, #32
        sbcs    x10, x24, x6
        add     x9, x2, x12
        sbcs    x6, x21, xzr
        lsr     x5, x9, #32
        sbcs    x21, x1, xzr
        sbc     x13, x3, xzr
        subs    x8, x5, x9
        sbc     x19, x9, xzr
        lsr     x12, x19, #32
        extr    x3, x19, x8, #32
        adds    x8, x12, x9
        adc     x1, xzr, xzr
        subs    x2, x17, x3
        sbcs    x12, x10, x8
        sbcs    x5, x6, x1
        sbcs    x3, x21, xzr
        sbcs    x19, x13, xzr
        sbc     x24, x9, xzr
        adds    x23, x15, x3
        adcs    x8, x7, x19
        adcs    x11, x20, x24
        adc     x9, x22, xzr
        add     x24, x9, #0x1
        lsl     x7, x24, #32
        subs    x21, x24, x7
        sbc     x10, x7, xzr
        adds    x6, x2, x21
        adcs    x7, x12, x10
        adcs    x24, x5, x24
        adcs    x13, x23, xzr
        adcs    x8, x8, xzr
        adcs    x15, x11, xzr
        csetm   x23, cc  // cc = lo, ul, last
        and     x11, x16, x23
        and     x20, x14, x23
        adds    x22, x6, x20
        eor     x3, x20, x23
        adcs    x5, x7, x3
        adcs    x14, x24, x11
        stp     x22, x5, [sp, #240]
        adcs    x5, x13, x23
        adcs    x12, x8, x23
        stp     x14, x5, [sp, #256]
        adc     x19, x15, x23
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
        csetm   x8, cc  // cc = lo, ul, last
        mov     x9, #0xffffffff                 // #4294967295
        and     x9, x9, x8
        adds    x0, x0, x9
        eor     x9, x9, x8
        adcs    x1, x1, x9
        mov     x9, #0xfffffffffffffffe         // #-2
        and     x9, x9, x8
        adcs    x2, x2, x9
        adcs    x3, x3, x8
        adcs    x4, x4, x8
        adc     x5, x5, x8
        stp     x0, x1, [x25]
        stp     x2, x3, [x25, #16]
        stp     x4, x5, [x25, #32]
        ldp     x0, x1, [sp, #192]
        mov     x6, #0xffffffff                 // #4294967295
        subs    x6, x6, x0
        mov     x7, #0xffffffff00000000         // #-4294967296
        sbcs    x7, x7, x1
        ldp     x0, x1, [sp, #208]
        mov     x8, #0xfffffffffffffffe         // #-2
        sbcs    x8, x8, x0
        mov     x13, #0xffffffffffffffff        // #-1
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
        add     x6, x6, #0x1
        ldp     x8, x9, [sp, #240]
        ldp     x10, x11, [sp, #256]
        mov     x14, #0x3                       // #3
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
        mul     x15, x14, x19
        umulh   x13, x14, x19
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
        csetm   x6, cc  // cc = lo, ul, last
        mov     x7, #0xffffffff                 // #4294967295
        and     x7, x7, x6
        adds    x0, x0, x7
        eor     x7, x7, x6
        adcs    x1, x1, x7
        mov     x7, #0xfffffffffffffffe         // #-2
        and     x7, x7, x6
        adcs    x2, x2, x7
        adcs    x3, x3, x6
        adcs    x4, x4, x6
        adc     x5, x5, x6
        stp     x0, x1, [x25, #48]
        stp     x2, x3, [x25, #64]
        stp     x4, x5, [x25, #80]
        ldp     x19, x20, [sp, #336]
        ldp     x21, x22, [sp, #352]
        ldp     x23, x24, [sp, #368]
        ldp     x25, x26, [sp, #384]
        ldp     x27, xzr, [sp, #400]
        CFI_INC_SP(416)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp384_montjscalarmul_p384_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
