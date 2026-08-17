// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery-Jacobian form scalar multiplication for P-256
// Input scalar[4], point[12]; output res[12]
//
// extern void p256_montjscalarmul_alt
//   (uint64_t res[static 12],
//    const uint64_t scalar[static 4],
//    const uint64_t point[static 12]);
//
// This function is a variant of its affine point version p256_scalarmul_alt.
// Here, input and output points are assumed to be in Jacobian form with
// their coordinates in the Montgomery domain. Thus, if priming indicates
// Montgomery form, x' = (2^256 * x) mod p_256 etc., each point argument
// is a triple (x',y',z') representing the affine point (x/z^2,y/z^3) when
// z' is nonzero or the point at infinity (group identity) if z' = 0.
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-256, returns a representation of n * P. If the result is the
// point at infinity (either because the input point was or because the
// scalar was a multiple of p_256) then the output is guaranteed to
// represent the point at infinity, i.e. to have its z coordinate zero.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------


#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjscalarmul_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjscalarmul_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjscalarmul_alt)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Safe copies of inputs (res lasts the whole code, point not so long)
// and additional values in variables, with some aliasing

#define res x19
#define sgn x20
#define j x20
#define point x21

// Intermediate variables on the stack.

#define scalarb sp, #(0*NUMSIZE)
#define acc sp, #(1*NUMSIZE)
#define tabent sp, #(4*NUMSIZE)

#define tab sp, #(7*NUMSIZE)

#define NSPACE 31*NUMSIZE

// Avoid using .rep for the sake of the BoringSSL/AWS-LC delocator,
// which doesn't accept repetitions, assembler macros etc.

#define selectblock(I)                          \
        cmp     x14, #(1*I) __LF                   \
        ldp     x12, x13, [x15] __LF               \
        csel    x0, x12, x0, eq __LF               \
        csel    x1, x13, x1, eq __LF               \
        ldp     x12, x13, [x15, #16] __LF          \
        csel    x2, x12, x2, eq __LF               \
        csel    x3, x13, x3, eq __LF               \
        ldp     x12, x13, [x15, #32] __LF          \
        csel    x4, x12, x4, eq __LF               \
        csel    x5, x13, x5, eq __LF               \
        ldp     x12, x13, [x15, #48] __LF          \
        csel    x6, x12, x6, eq __LF               \
        csel    x7, x13, x7, eq __LF               \
        ldp     x12, x13, [x15, #64] __LF          \
        csel    x8, x12, x8, eq __LF               \
        csel    x9, x13, x9, eq __LF               \
        ldp     x12, x13, [x15, #80] __LF          \
        csel    x10, x12, x10, eq __LF             \
        csel    x11, x13, x11, eq __LF             \
        add     x15, x15, #96

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(p256_montjscalarmul_alt):
        CFI_START

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x30)
        CFI_DEC_SP(NSPACE)

// Preserve the "res" and "point" input arguments. We load and process the
// scalar immediately so we don't bother preserving that input argument.
// Also, "point" is only needed early on and so its register gets re-used.

        mov     res, x0
        mov     point, x2

// Load the digits of group order n_256 = [x12;x13;x14;x15]

        movbig(x12, #0xf3b9, #0xcac2, #0xfc63, #0x2551)
        movbig(x13, #0xbce6, #0xfaad, #0xa717, #0x9e84)
        mov     x14, #0xffffffffffffffff
        mov     x15, #0xffffffff00000000

// First, reduce the input scalar mod n_256, i.e. conditionally subtract n_256

        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]

        subs    x6, x2, x12
        sbcs    x7, x3, x13
        sbcs    x8, x4, x14
        sbcs    x9, x5, x15

        csel    x2, x2, x6, cc
        csel    x3, x3, x7, cc
        csel    x4, x4, x8, cc
        csel    x5, x5, x9, cc

// Now if the top bit of the reduced scalar is set, negate it mod n_256,
// i.e. do n |-> n_256 - n. Remember the sign as "sgn" so we can
// correspondingly negate the point below.

        subs    x6, x12, x2
        sbcs    x7, x13, x3
        sbcs    x8, x14, x4
        sbc     x9, x15, x5

        tst     x5, #0x8000000000000000
        csel    x2, x2, x6, eq
        csel    x3, x3, x7, eq
        csel    x4, x4, x8, eq
        csel    x5, x5, x9, eq
        cset    sgn, ne

// In either case then add the recoding constant 0x08888...888 to allow
// signed digits.

        mov     x6, 0x8888888888888888
        adds    x2, x2, x6
        adcs    x3, x3, x6
        bic     x7, x6, #0xF000000000000000
        adcs    x4, x4, x6
        adc     x5, x5, x7

        stp     x2, x3, [scalarb]
        stp     x4, x5, [scalarb+16]

// Set the tab[0] table entry to the input point = 1 * P, except
// that we negate it if the top bit of the scalar was set. This
// negation takes care over the y = 0 case to maintain all the
// coordinates < p_256 throughout, even though triples (x,y,z)
// with y = 0 can only represent a point on the curve when z = 0
// and it represents the point at infinity regardless of x and y.

        ldp     x0, x1, [point]
        stp     x0, x1, [tab]
        ldp     x2, x3, [point, #16]
        stp     x2, x3, [tab+16]

        ldp     x4, x5, [point, #32]
        ldp     x6, x7, [point, #48]

        mov     x0, 0xffffffffffffffff
        subs    x0, x0, x4
        mov     x1, 0x00000000ffffffff
        sbcs    x1, x1, x5
        mov     x3, 0xffffffff00000001
        sbcs    x2, xzr, x6
        sbc     x3, x3, x7

        orr     x8, x4, x5
        orr     x9, x6, x7
        orr     x8, x8, x9
        cmp     x8, xzr
        ccmp    sgn, xzr, #4, ne
        csel    x4, x0, x4, ne
        csel    x5, x1, x5, ne
        csel    x6, x2, x6, ne
        csel    x7, x3, x7, ne

        stp     x4, x5, [tab+32]
        stp     x6, x7, [tab+48]

        ldp     x0, x1, [point, #64]
        stp     x0, x1, [tab+64]
        ldp     x2, x3, [point, #80]
        stp     x2, x3, [tab+80]

// Compute and record tab[1] = 2 * p, ..., tab[7] = 8 * P

        add     x0, tab+96*1
        add     x1, tab
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

        add     x0, tab+96*2
        add     x1, tab+96*1
        add     x2, tab
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjadd)

        add     x0, tab+96*3
        add     x1, tab+96*1
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

        add     x0, tab+96*4
        add     x1, tab+96*3
        add     x2, tab
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjadd)

        add     x0, tab+96*5
        add     x1, tab+96*2
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

        add     x0, tab+96*6
        add     x1, tab+96*5
        add     x2, tab
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjadd)

        add     x0, tab+96*7
        add     x1, tab+96*3
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

// Initialize the accumulator as a table entry for top 4 bits (unrecoded)

        ldr     x14, [scalarb+24]
        lsr     x14, x14, #60

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
        add     x15, tab

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)

        stp     x0, x1, [acc]
        stp     x2, x3, [acc+16]
        stp     x4, x5, [acc+32]
        stp     x6, x7, [acc+48]
        stp     x8, x9, [acc+64]
        stp     x10, x11, [acc+80]

        mov     j, #252

// Main loop over size-4 bitfields: double 4 times then add signed digit

Lp256_montjscalarmul_alt_mainloop:
        sub     j, j, #4

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjdouble)

        lsr     x2, j, #6
        ldr     x14, [sp, x2, lsl #3]   // Exploits scalarb = sp exactly
        lsr     x14, x14, j
        and     x14, x14, #15

        subs    x14, x14, #8
        cset    x16, lo                 // x16 = sign of digit (1 = negative)
        cneg    x14, x14, lo            // x14 = absolute value of digit

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
        add     x15, tab

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)

// Store it to "tabent" with the y coordinate optionally negated
// Again, do it carefully to give coordinates < p_256 even in
// the degenerate case y = 0 (when z = 0 for points on the curve).

        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]

        mov     x0, 0xffffffffffffffff
        subs    x0, x0, x4
        mov     x1, 0x00000000ffffffff
        sbcs    x1, x1, x5
        mov     x3, 0xffffffff00000001
        sbcs    x2, xzr, x6
        sbc     x3, x3, x7

        orr     x12, x4, x5
        orr     x13, x6, x7
        orr     x12, x12, x13
        cmp     x12, xzr
        ccmp    x16, xzr, #4, ne
        csel    x4, x0, x4, ne
        csel    x5, x1, x5, ne
        csel    x6, x2, x6, ne
        csel    x7, x3, x7, ne

        stp     x4, x5, [tabent+32]
        stp     x6, x7, [tabent+48]
        stp     x8, x9, [tabent+64]
        stp     x10, x11, [tabent+80]

        add     x0, acc
        add     x1, acc
        add     x2, tabent
        CFI_BL(Lp256_montjscalarmul_alt_p256_montjadd)

        cbnz    j, Lp256_montjscalarmul_alt_mainloop

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

// Restore stack and registers and return

        CFI_INC_SP(NSPACE)
        CFI_POP2(x21,x30)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_montjscalarmul_alt)

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_montjscalarmul_alt_p256_montjadd)

Lp256_montjscalarmul_alt_p256_montjadd:
        CFI_START
        CFI_DEC_SP(224)
        mov     x15, x0
        mov     x16, x1
        mov     x17, x2
        ldp     x2, x3, [x16, #64]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x16, #80]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x8, x3
        umulh   x8, x8, x3
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x9, x3
        umulh   x9, x9, x3
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x10, x3
        umulh   x10, x10, x3
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x11, x3
        umulh   x11, x11, x3
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        mov     x2, #0xffffffffffffffff
        csel    x2, xzr, x2, cc
        mov     x3, #0xffffffff
        csel    x3, xzr, x3, cc
        mov     x5, #0xffffffff00000001
        csel    x5, xzr, x5, cc
        subs    x8, x8, x2
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbc     x11, x11, x5
        stp     x8, x9, [sp]
        stp     x10, x11, [sp, #16]
        ldp     x2, x3, [x17, #64]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x17, #80]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x8, x3
        umulh   x8, x8, x3
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x9, x3
        umulh   x9, x9, x3
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x10, x3
        umulh   x10, x10, x3
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x11, x3
        umulh   x11, x11, x3
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        mov     x2, #0xffffffffffffffff
        csel    x2, xzr, x2, cc
        mov     x3, #0xffffffff
        csel    x3, xzr, x3, cc
        mov     x5, #0xffffffff00000001
        csel    x5, xzr, x5, cc
        subs    x8, x8, x2
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbc     x11, x11, x5
        stp     x8, x9, [sp, #160]
        stp     x10, x11, [sp, #176]
        ldp     x3, x4, [x17, #64]
        ldp     x7, x8, [x16, #32]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x16, #48]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [x17, #80]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #192]
        stp     x14, x0, [sp, #208]
        ldp     x3, x4, [x16, #64]
        ldp     x7, x8, [x17, #32]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x17, #48]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [x16, #80]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #32]
        stp     x14, x0, [sp, #48]
        ldp     x3, x4, [sp]
        ldp     x7, x8, [x17]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x17, #16]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #16]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #64]
        stp     x14, x0, [sp, #80]
        ldp     x3, x4, [sp, #160]
        ldp     x7, x8, [x16]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x16, #16]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #176]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #128]
        stp     x14, x0, [sp, #144]
        ldp     x3, x4, [sp]
        ldp     x7, x8, [sp, #32]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #48]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #16]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #32]
        stp     x14, x0, [sp, #48]
        ldp     x3, x4, [sp, #160]
        ldp     x7, x8, [sp, #192]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #208]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #176]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #192]
        stp     x14, x0, [sp, #208]
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [sp, #128]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [sp, #144]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        mov     x4, #0xffffffff00000001
        and     x4, x4, x3
        adc     x8, x8, x4
        stp     x5, x6, [sp, #160]
        stp     x7, x8, [sp, #176]
        ldp     x5, x6, [sp, #32]
        ldp     x4, x3, [sp, #192]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #48]
        ldp     x4, x3, [sp, #208]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        mov     x4, #0xffffffff00000001
        and     x4, x4, x3
        adc     x8, x8, x4
        stp     x5, x6, [sp, #32]
        stp     x7, x8, [sp, #48]
        ldp     x2, x3, [sp, #160]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #176]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x8, x3
        umulh   x8, x8, x3
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x9, x3
        umulh   x9, x9, x3
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x10, x3
        umulh   x10, x10, x3
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x11, x3
        umulh   x11, x11, x3
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        mov     x2, #0xffffffffffffffff
        csel    x2, xzr, x2, cc
        mov     x3, #0xffffffff
        csel    x3, xzr, x3, cc
        mov     x5, #0xffffffff00000001
        csel    x5, xzr, x5, cc
        subs    x8, x8, x2
        sbcs    x9, x9, x3
        sbcs    x10, x10, xzr
        sbc     x11, x11, x5
        stp     x8, x9, [sp, #96]
        stp     x10, x11, [sp, #112]
        ldp     x2, x3, [sp, #32]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #48]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x8, x3
        umulh   x8, x8, x3
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x9, x3
        umulh   x9, x9, x3
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x10, x3
        umulh   x10, x10, x3
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mov     x3, #0xffffffff00000001
        mul     x2, x11, x3
        umulh   x11, x11, x3
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        cset    x2, cs
        mov     x3, #0xffffffff
        mov     x5, #0xffffffff00000001
        adds    x12, x8, #0x1
        sbcs    x13, x9, x3
        sbcs    x14, x10, xzr
        sbcs    x7, x11, x5
        sbcs    xzr, x2, xzr
        csel    x8, x8, x12, cc
        csel    x9, x9, x13, cc
        csel    x10, x10, x14, cc
        csel    x11, x11, x7, cc
        stp     x8, x9, [sp]
        stp     x10, x11, [sp, #16]
        ldp     x3, x4, [sp, #96]
        ldp     x7, x8, [sp, #128]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #144]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #112]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #128]
        stp     x14, x0, [sp, #144]
        ldp     x3, x4, [sp, #96]
        ldp     x7, x8, [sp, #64]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #80]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #112]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #64]
        stp     x14, x0, [sp, #80]
        ldp     x5, x6, [sp]
        ldp     x4, x3, [sp, #128]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #16]
        ldp     x4, x3, [sp, #144]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        mov     x4, #0xffffffff00000001
        and     x4, x4, x3
        adc     x8, x8, x4
        stp     x5, x6, [sp]
        stp     x7, x8, [sp, #16]
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [sp, #128]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [sp, #144]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        mov     x4, #0xffffffff00000001
        and     x4, x4, x3
        adc     x8, x8, x4
        stp     x5, x6, [sp, #96]
        stp     x7, x8, [sp, #112]
        ldp     x3, x4, [sp, #160]
        ldp     x7, x8, [x16, #64]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x16, #80]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #176]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #160]
        stp     x14, x0, [sp, #176]
        ldp     x5, x6, [sp]
        ldp     x4, x3, [sp, #64]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #16]
        ldp     x4, x3, [sp, #80]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        mov     x4, #0xffffffff00000001
        and     x4, x4, x3
        adc     x8, x8, x4
        stp     x5, x6, [sp]
        stp     x7, x8, [sp, #16]
        ldp     x5, x6, [sp, #128]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #144]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        mov     x4, #0xffffffff00000001
        and     x4, x4, x3
        adc     x8, x8, x4
        stp     x5, x6, [sp, #128]
        stp     x7, x8, [sp, #144]
        ldp     x3, x4, [sp, #96]
        ldp     x7, x8, [sp, #192]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #208]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #112]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #96]
        stp     x14, x0, [sp, #112]
        ldp     x3, x4, [sp, #160]
        ldp     x7, x8, [x17, #64]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x17, #80]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #176]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #160]
        stp     x14, x0, [sp, #176]
        ldp     x3, x4, [sp, #32]
        ldp     x7, x8, [sp, #128]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #144]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #48]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #128]
        stp     x14, x0, [sp, #144]
        ldp     x5, x6, [sp, #128]
        ldp     x4, x3, [sp, #96]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #144]
        ldp     x4, x3, [sp, #112]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        mov     x4, #0xffffffff
        and     x4, x4, x3
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        mov     x4, #0xffffffff00000001
        and     x4, x4, x3
        adc     x8, x8, x4
        stp     x5, x6, [sp, #128]
        stp     x7, x8, [sp, #144]
        ldp     x0, x1, [x16, #64]
        ldp     x2, x3, [x16, #80]
        orr     x12, x0, x1
        orr     x13, x2, x3
        orr     x12, x12, x13
        cmp     x12, xzr
        cset    x12, ne
        ldp     x4, x5, [x17, #64]
        ldp     x6, x7, [x17, #80]
        orr     x13, x4, x5
        orr     x14, x6, x7
        orr     x13, x13, x14
        cmp     x13, xzr
        cset    x13, ne
        cmp     x13, x12
        ldp     x8, x9, [sp, #160]
        csel    x8, x0, x8, cc
        csel    x9, x1, x9, cc
        csel    x8, x4, x8, hi
        csel    x9, x5, x9, hi
        ldp     x10, x11, [sp, #176]
        csel    x10, x2, x10, cc
        csel    x11, x3, x11, cc
        csel    x10, x6, x10, hi
        csel    x11, x7, x11, hi
        ldp     x12, x13, [x16]
        ldp     x0, x1, [sp]
        csel    x0, x12, x0, cc
        csel    x1, x13, x1, cc
        ldp     x12, x13, [x17]
        csel    x0, x12, x0, hi
        csel    x1, x13, x1, hi
        ldp     x12, x13, [x16, #16]
        ldp     x2, x3, [sp, #16]
        csel    x2, x12, x2, cc
        csel    x3, x13, x3, cc
        ldp     x12, x13, [x17, #16]
        csel    x2, x12, x2, hi
        csel    x3, x13, x3, hi
        ldp     x12, x13, [x16, #32]
        ldp     x4, x5, [sp, #128]
        csel    x4, x12, x4, cc
        csel    x5, x13, x5, cc
        ldp     x12, x13, [x17, #32]
        csel    x4, x12, x4, hi
        csel    x5, x13, x5, hi
        ldp     x12, x13, [x16, #48]
        ldp     x6, x7, [sp, #144]
        csel    x6, x12, x6, cc
        csel    x7, x13, x7, cc
        ldp     x12, x13, [x17, #48]
        csel    x6, x12, x6, hi
        csel    x7, x13, x7, hi
        stp     x0, x1, [x15]
        stp     x2, x3, [x15, #16]
        stp     x4, x5, [x15, #32]
        stp     x6, x7, [x15, #48]
        stp     x8, x9, [x15, #64]
        stp     x10, x11, [x15, #80]
        CFI_INC_SP(224)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_montjscalarmul_alt_p256_montjadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_montjscalarmul_alt_p256_montjdouble)

Lp256_montjscalarmul_alt_p256_montjdouble:
        CFI_START
        CFI_DEC_SP(192)
        mov     x15, x0
        mov     x16, x1
        ldp     x2, x3, [x16, #64]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x16, #80]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        mov     x5, #0xffffffff00000001
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mul     x2, x8, x5
        umulh   x8, x8, x5
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mul     x2, x9, x5
        umulh   x9, x9, x5
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mul     x2, x10, x5
        umulh   x10, x10, x5
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mul     x2, x11, x5
        umulh   x11, x11, x5
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        cset    x2, cs
        mov     x3, #0xffffffff
        adds    x12, x8, #0x1
        sbcs    x13, x9, x3
        sbcs    x14, x10, xzr
        sbcs    x7, x11, x5
        sbcs    xzr, x2, xzr
        csel    x8, x8, x12, cc
        csel    x9, x9, x13, cc
        csel    x10, x10, x14, cc
        csel    x11, x11, x7, cc
        stp     x8, x9, [sp]
        stp     x10, x11, [sp, #16]
        ldp     x2, x3, [x16, #32]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x16, #48]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        mov     x5, #0xffffffff00000001
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mul     x2, x8, x5
        umulh   x8, x8, x5
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mul     x2, x9, x5
        umulh   x9, x9, x5
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mul     x2, x10, x5
        umulh   x10, x10, x5
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mul     x2, x11, x5
        umulh   x11, x11, x5
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        cset    x2, cs
        mov     x3, #0xffffffff
        adds    x12, x8, #0x1
        sbcs    x13, x9, x3
        sbcs    x14, x10, xzr
        sbcs    x7, x11, x5
        sbcs    xzr, x2, xzr
        csel    x8, x8, x12, cc
        csel    x9, x9, x13, cc
        csel    x10, x10, x14, cc
        csel    x11, x11, x7, cc
        stp     x8, x9, [sp, #32]
        stp     x10, x11, [sp, #48]
        ldp     x5, x6, [x16]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [x16, #16]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [sp, #96]
        stp     x7, x8, [sp, #112]
        ldp     x5, x6, [x16]
        ldp     x4, x3, [sp]
        adds    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x16, #16]
        ldp     x4, x3, [sp, #16]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        csetm   x3, cs
        subs    x5, x5, x3
        and     x1, x3, #0xffffffff
        sbcs    x6, x6, x1
        sbcs    x7, x7, xzr
        and     x2, x3, #0xffffffff00000001
        sbc     x8, x8, x2
        stp     x5, x6, [sp, #64]
        stp     x7, x8, [sp, #80]
        ldp     x3, x4, [sp, #64]
        ldp     x7, x8, [sp, #96]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #112]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #80]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #96]
        stp     x14, x0, [sp, #112]
        ldp     x5, x6, [x16, #32]
        ldp     x4, x3, [x16, #64]
        adds    x5, x5, x4
        adcs    x6, x6, x3
        ldp     x7, x8, [x16, #48]
        ldp     x4, x3, [x16, #80]
        adcs    x7, x7, x4
        adcs    x8, x8, x3
        adc     x3, xzr, xzr
        cmn     x5, #0x1
        mov     x4, #0xffffffff
        sbcs    xzr, x6, x4
        sbcs    xzr, x7, xzr
        mov     x4, #0xffffffff00000001
        sbcs    xzr, x8, x4
        adcs    x3, x3, xzr
        csetm   x3, ne
        subs    x5, x5, x3
        and     x4, x3, #0xffffffff
        sbcs    x6, x6, x4
        sbcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        sbc     x8, x8, x4
        stp     x5, x6, [sp, #64]
        stp     x7, x8, [sp, #80]
        ldp     x3, x4, [x16]
        ldp     x7, x8, [sp, #32]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #48]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [x16, #16]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #128]
        stp     x14, x0, [sp, #144]
        ldp     x2, x3, [sp, #96]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #112]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        mov     x5, #0xffffffff00000001
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mul     x2, x8, x5
        umulh   x8, x8, x5
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mul     x2, x9, x5
        umulh   x9, x9, x5
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mul     x2, x10, x5
        umulh   x10, x10, x5
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mul     x2, x11, x5
        umulh   x11, x11, x5
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        cset    x2, cs
        mov     x3, #0xffffffff
        adds    x12, x8, #0x1
        sbcs    x13, x9, x3
        sbcs    x14, x10, xzr
        sbcs    x7, x11, x5
        sbcs    xzr, x2, xzr
        csel    x8, x8, x12, cc
        csel    x9, x9, x13, cc
        csel    x10, x10, x14, cc
        csel    x11, x11, x7, cc
        stp     x8, x9, [sp, #160]
        stp     x10, x11, [sp, #176]
        ldp     x2, x3, [sp, #64]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #80]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        mov     x5, #0xffffffff00000001
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mul     x2, x8, x5
        umulh   x8, x8, x5
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mul     x2, x9, x5
        umulh   x9, x9, x5
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mul     x2, x10, x5
        umulh   x10, x10, x5
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mul     x2, x11, x5
        umulh   x11, x11, x5
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        cset    x2, cs
        mov     x3, #0xffffffff
        adds    x12, x8, #0x1
        sbcs    x13, x9, x3
        sbcs    x14, x10, xzr
        sbcs    x7, x11, x5
        sbcs    xzr, x2, xzr
        csel    x8, x8, x12, cc
        csel    x9, x9, x13, cc
        csel    x10, x10, x14, cc
        csel    x11, x11, x7, cc
        stp     x8, x9, [sp, #64]
        stp     x10, x11, [sp, #80]
        mov     x1, #0x9
        mov     x2, #0xffffffffffffffff
        ldp     x9, x10, [sp, #160]
        subs    x9, x2, x9
        mov     x2, #0xffffffff
        sbcs    x10, x2, x10
        ldp     x11, x12, [sp, #176]
        ngcs    x11, x11
        mov     x2, #0xffffffff00000001
        sbc     x12, x2, x12
        mul     x3, x1, x9
        mul     x4, x1, x10
        mul     x5, x1, x11
        mul     x6, x1, x12
        umulh   x9, x1, x9
        umulh   x10, x1, x10
        umulh   x11, x1, x11
        umulh   x7, x1, x12
        adds    x4, x4, x9
        adcs    x5, x5, x10
        adcs    x6, x6, x11
        adc     x7, x7, xzr
        mov     x1, #0xc
        ldp     x9, x10, [sp, #128]
        mul     x8, x9, x1
        umulh   x9, x9, x1
        adds    x3, x3, x8
        mul     x8, x10, x1
        umulh   x10, x10, x1
        adcs    x4, x4, x8
        ldp     x11, x12, [sp, #144]
        mul     x8, x11, x1
        umulh   x11, x11, x1
        adcs    x5, x5, x8
        mul     x8, x12, x1
        umulh   x12, x12, x1
        adcs    x6, x6, x8
        adc     x7, x7, xzr
        adds    x4, x4, x9
        adcs    x5, x5, x10
        adcs    x6, x6, x11
        adc     x7, x7, x12
        add     x8, x7, #0x1
        lsl     x10, x8, #32
        adds    x6, x6, x10
        adc     x7, x7, xzr
        neg     x9, x8
        sub     x10, x10, #0x1
        subs    x3, x3, x9
        sbcs    x4, x4, x10
        sbcs    x5, x5, xzr
        sbcs    x6, x6, x8
        sbc     x8, x7, x8
        adds    x3, x3, x8
        and     x9, x8, #0xffffffff
        adcs    x4, x4, x9
        adcs    x5, x5, xzr
        neg     x10, x9
        adc     x6, x6, x10
        stp     x3, x4, [sp, #160]
        stp     x5, x6, [sp, #176]
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [sp]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [sp, #16]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [sp, #64]
        stp     x7, x8, [sp, #80]
        ldp     x2, x3, [sp, #32]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [sp, #48]
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x6, x2, x4
        umulh   x7, x2, x4
        adds    x10, x10, x6
        adcs    x11, x11, x7
        mul     x6, x3, x4
        umulh   x7, x3, x4
        adc     x7, x7, xzr
        adds    x11, x11, x6
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x7
        mul     x6, x3, x5
        umulh   x7, x3, x5
        adc     x7, x7, xzr
        adds    x12, x12, x6
        adcs    x13, x13, x7
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x7, cs
        umulh   x6, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x6
        mul     x6, x3, x3
        adcs    x10, x10, x6
        umulh   x6, x3, x3
        adcs    x11, x11, x6
        mul     x6, x4, x4
        adcs    x12, x12, x6
        umulh   x6, x4, x4
        adcs    x13, x13, x6
        mul     x6, x5, x5
        adcs    x14, x14, x6
        umulh   x6, x5, x5
        adc     x7, x7, x6
        mov     x5, #0xffffffff00000001
        adds    x9, x9, x8, lsl #32
        lsr     x3, x8, #32
        adcs    x10, x10, x3
        mul     x2, x8, x5
        umulh   x8, x8, x5
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x3, x9, #32
        adcs    x11, x11, x3
        mul     x2, x9, x5
        umulh   x9, x9, x5
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x3, x10, #32
        adcs    x8, x8, x3
        mul     x2, x10, x5
        umulh   x10, x10, x5
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x3, x11, #32
        adcs    x9, x9, x3
        mul     x2, x11, x5
        umulh   x11, x11, x5
        adcs    x10, x10, x2
        adc     x11, x11, xzr
        adds    x8, x8, x12
        adcs    x9, x9, x13
        adcs    x10, x10, x14
        adcs    x11, x11, x7
        cset    x2, cs
        mov     x3, #0xffffffff
        adds    x12, x8, #0x1
        sbcs    x13, x9, x3
        sbcs    x14, x10, xzr
        sbcs    x7, x11, x5
        sbcs    xzr, x2, xzr
        csel    x8, x8, x12, cc
        csel    x9, x9, x13, cc
        csel    x10, x10, x14, cc
        csel    x11, x11, x7, cc
        stp     x8, x9, [sp]
        stp     x10, x11, [sp, #16]
        ldp     x3, x4, [sp, #160]
        ldp     x7, x8, [sp, #96]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [sp, #112]
        mul     x11, x3, x9
        umulh   x0, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x1, x3, x10
        adcs    x0, x0, x11
        adc     x1, x1, xzr
        ldp     x5, x6, [sp, #176]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x0, x0, x11
        mul     x11, x4, x10
        adcs    x1, x1, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x0, x0, x11
        umulh   x11, x4, x9
        adcs    x1, x1, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x0, x0, x11
        mul     x11, x5, x9
        adcs    x1, x1, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x0, x0, x11
        umulh   x11, x5, x8
        adcs    x1, x1, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x0, x0, x11
        mul     x11, x6, x8
        adcs    x1, x1, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        mov     x10, #0xffffffff00000001
        adds    x13, x13, x12, lsl #32
        lsr     x11, x12, #32
        adcs    x14, x14, x11
        mul     x11, x12, x10
        umulh   x12, x12, x10
        adcs    x0, x0, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x1, x1, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x0, x0, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x0, x0, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x0, lsl #32
        lsr     x11, x0, #32
        adcs    x13, x13, x11
        mul     x11, x0, x10
        umulh   x0, x0, x10
        adcs    x14, x14, x11
        adc     x0, x0, xzr
        adds    x12, x12, x1
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x0, x0, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x1, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x0, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x1, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x0, x0, x5, cc
        stp     x12, x13, [sp, #96]
        stp     x14, x0, [sp, #112]
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [sp, #32]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [sp, #48]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [x15, #64]
        stp     x7, x8, [x15, #80]
        ldp     x1, x2, [sp, #128]
        lsl     x0, x1, #2
        ldp     x6, x7, [sp, #160]
        subs    x0, x0, x6
        extr    x1, x2, x1, #62
        sbcs    x1, x1, x7
        ldp     x3, x4, [sp, #144]
        extr    x2, x3, x2, #62
        ldp     x6, x7, [sp, #176]
        sbcs    x2, x2, x6
        extr    x3, x4, x3, #62
        sbcs    x3, x3, x7
        lsr     x4, x4, #62
        sbc     x4, x4, xzr
        add     x5, x4, #0x1
        lsl     x8, x5, #32
        negs    x6, x8
        ngcs    x7, xzr
        sbc     x8, x8, x5
        adds    x0, x0, x5
        adcs    x1, x1, x6
        adcs    x2, x2, x7
        adcs    x3, x3, x8
        csetm   x5, cc
        adds    x0, x0, x5
        and     x6, x5, #0xffffffff
        adcs    x1, x1, x6
        adcs    x2, x2, xzr
        neg     x7, x6
        adc     x3, x3, x7
        stp     x0, x1, [x15]
        stp     x2, x3, [x15, #16]
        mov     x1, #0x8
        mov     x2, #0xffffffffffffffff
        ldp     x9, x10, [sp]
        subs    x9, x2, x9
        mov     x2, #0xffffffff
        sbcs    x10, x2, x10
        ldp     x11, x12, [sp, #16]
        ngcs    x11, x11
        mov     x2, #0xffffffff00000001
        sbc     x12, x2, x12
        lsl     x3, x9, #3
        extr    x4, x10, x9, #61
        extr    x5, x11, x10, #61
        extr    x6, x12, x11, #61
        lsr     x7, x12, #61
        mov     x1, #0x3
        ldp     x9, x10, [sp, #96]
        mul     x8, x9, x1
        umulh   x9, x9, x1
        adds    x3, x3, x8
        mul     x8, x10, x1
        umulh   x10, x10, x1
        adcs    x4, x4, x8
        ldp     x11, x12, [sp, #112]
        mul     x8, x11, x1
        umulh   x11, x11, x1
        adcs    x5, x5, x8
        mul     x8, x12, x1
        umulh   x12, x12, x1
        adcs    x6, x6, x8
        adc     x7, x7, xzr
        adds    x4, x4, x9
        adcs    x5, x5, x10
        adcs    x6, x6, x11
        adc     x7, x7, x12
        add     x8, x7, #0x1
        lsl     x10, x8, #32
        adds    x6, x6, x10
        adc     x7, x7, xzr
        neg     x9, x8
        sub     x10, x10, #0x1
        subs    x3, x3, x9
        sbcs    x4, x4, x10
        sbcs    x5, x5, xzr
        sbcs    x6, x6, x8
        sbc     x8, x7, x8
        adds    x3, x3, x8
        and     x9, x8, #0xffffffff
        adcs    x4, x4, x9
        adcs    x5, x5, xzr
        neg     x10, x9
        adc     x6, x6, x10
        stp     x3, x4, [x15, #32]
        stp     x5, x6, [x15, #48]
        CFI_INC_SP(192)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_montjscalarmul_alt_p256_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
