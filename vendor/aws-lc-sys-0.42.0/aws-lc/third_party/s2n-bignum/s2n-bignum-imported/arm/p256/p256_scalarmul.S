// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Scalar multiplication for P-256
// Input scalar[4], point[8]; output res[8]
//
// extern void p256_scalarmul
//   (uint64_t res[static 8],
//    const uint64_t scalar[static 4],
//    const uint64_t point[static 8]);
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-256, returns the point (X,Y) = n * P. The input and output
// are affine points, and in the case of the point at infinity as
// the result, (0,0) is returned.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------


#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_scalarmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_scalarmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_scalarmul)


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

// Intermediate variables on the stack. The last z2, z3 values can
// safely be overlaid on the table, which is no longer needed at the end.

#define scalarb sp, #(0*NUMSIZE)
#define acc sp, #(1*NUMSIZE)
#define tabent sp, #(4*NUMSIZE)

#define tab sp, #(7*NUMSIZE)

#define z2 sp, #(7*NUMSIZE)
#define z3 sp, #(8*NUMSIZE)

#define NSPACE 31*NUMSIZE

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(p256_scalarmul):
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

// Set the tab[0] table entry to Montgomery-Jacobian point = 1 * P
// The z coordinate is just the Montgomery form of the constant 1.

        add     x0, tab
        mov     x1, point
        CFI_BL(Lp256_scalarmul_local_tomont_p256)

        add     x1, point, #32
        add     x0, tab+32
        CFI_BL(Lp256_scalarmul_local_tomont_p256)

        mov     x0, #0x0000000000000001
        mov     x1, #0xffffffff00000000
        stp     x0, x1, [tab+64]
        mov     x2, #0xffffffffffffffff
        mov     x3, #0x00000000fffffffe
        stp     x2, x3, [tab+80]

// If the top bit of the scalar was set, negate (y coordinate of) the point

        ldp     x4, x5, [tab+32]
        ldp     x6, x7, [tab+48]

        mov     x0, 0xffffffffffffffff
        subs    x0, x0, x4
        mov     x1, 0x00000000ffffffff
        sbcs    x1, x1, x5
        mov     x3, 0xffffffff00000001
        sbcs    x2, xzr, x6
        sbc     x3, x3, x7

        cmp     sgn, xzr
        csel    x4, x0, x4, ne
        csel    x5, x1, x5, ne
        csel    x6, x2, x6, ne
        csel    x7, x3, x7, ne

        stp     x4, x5, [tab+32]
        stp     x6, x7, [tab+48]

// Compute and record tab[1] = 2 * p, ..., tab[7] = 8 * P

        add     x0, tab+96*1
        add     x1, tab
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

        add     x0, tab+96*2
        add     x1, tab+96*1
        add     x2, tab
        CFI_BL(Lp256_scalarmul_local_p256_montjmixadd)

        add     x0, tab+96*3
        add     x1, tab+96*1
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

        add     x0, tab+96*4
        add     x1, tab+96*3
        add     x2, tab
        CFI_BL(Lp256_scalarmul_local_p256_montjmixadd)

        add     x0, tab+96*5
        add     x1, tab+96*2
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

        add     x0, tab+96*6
        add     x1, tab+96*5
        add     x2, tab
        CFI_BL(Lp256_scalarmul_local_p256_montjmixadd)

        add     x0, tab+96*7
        add     x1, tab+96*3
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

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

        .set i, 1
.rep 8
        cmp     x14, #i
        ldp     x12, x13, [x15]
        csel    x0, x12, x0, eq
        csel    x1, x13, x1, eq
        ldp     x12, x13, [x15, #16]
        csel    x2, x12, x2, eq
        csel    x3, x13, x3, eq
        ldp     x12, x13, [x15, #32]
        csel    x4, x12, x4, eq
        csel    x5, x13, x5, eq
        ldp     x12, x13, [x15, #48]
        csel    x6, x12, x6, eq
        csel    x7, x13, x7, eq
        ldp     x12, x13, [x15, #64]
        csel    x8, x12, x8, eq
        csel    x9, x13, x9, eq
        ldp     x12, x13, [x15, #80]
        csel    x10, x12, x10, eq
        csel    x11, x13, x11, eq
        add     x15, x15, #96
        .set    i, (i+1)
.endr
        stp     x0, x1, [acc]
        stp     x2, x3, [acc+16]
        stp     x4, x5, [acc+32]
        stp     x6, x7, [acc+48]
        stp     x8, x9, [acc+64]
        stp     x10, x11, [acc+80]

        mov     j, #252

// Main loop over size-4 bitfields: double 4 times then add signed digit

Lp256_scalarmul_loop:
        sub     j, j, #4

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

        add     x0, acc
        add     x1, acc
        CFI_BL(Lp256_scalarmul_local_p256_montjdouble)

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
        .set i, 1
.rep 8
        cmp     x14, #i
        ldp     x12, x13, [x15]
        csel    x0, x12, x0, eq
        csel    x1, x13, x1, eq
        ldp     x12, x13, [x15, #16]
        csel    x2, x12, x2, eq
        csel    x3, x13, x3, eq
        ldp     x12, x13, [x15, #32]
        csel    x4, x12, x4, eq
        csel    x5, x13, x5, eq
        ldp     x12, x13, [x15, #48]
        csel    x6, x12, x6, eq
        csel    x7, x13, x7, eq
        ldp     x12, x13, [x15, #64]
        csel    x8, x12, x8, eq
        csel    x9, x13, x9, eq
        ldp     x12, x13, [x15, #80]
        csel    x10, x12, x10, eq
        csel    x11, x13, x11, eq
        add     x15, x15, #96
        .set    i, (i+1)
.endr

// Store it to "tabent" with the y coordinate optionally negated

        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]

        mov     x0, 0xffffffffffffffff
        subs    x0, x0, x4
        mov     x1, 0x00000000ffffffff
        sbcs    x1, x1, x5
        mov     x3, 0xffffffff00000001
        sbcs    x2, xzr, x6
        sbc     x3, x3, x7

        cmp     x16, xzr
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
        CFI_BL(Lp256_scalarmul_local_p256_montjadd)

        cbnz    j, Lp256_scalarmul_loop

// That's the end of the main loop, and we just need to translate
// back from the Jacobian representation to affine. First of all,
// let z2 = 1/z^2 and z3 = 1/z^3, both without Montgomery form

        add     x0, z2
        add     x1, acc+64
        CFI_BL(Lp256_scalarmul_local_montsqr_p256)

        add     x0, z3
        add     x2, z2
        add     x1, acc+64
        CFI_BL(Lp256_scalarmul_local_montmul_p256)

        add     x0, z2
        add     x1, z3
        CFI_BL(Lp256_scalarmul_local_demont_p256)

        add     x0, z3
        add     x1, z2
        CFI_BL(Lp256_scalarmul_local_inv_p256)

        add     x0, z2
        add     x2, z3
        add     x1, acc+64
        CFI_BL(Lp256_scalarmul_local_montmul_p256)

// Convert back from Jacobian (X,Y,Z) |-> (X/Z^2, Y/Z^3)

        add     x1, acc
        add     x2, z2
        mov     x0, res
        CFI_BL(Lp256_scalarmul_local_montmul_p256)

        add     x0, res, #32
        add     x1, acc+32
        add     x2, z3
        CFI_BL(Lp256_scalarmul_local_montmul_p256)

// Restore stack and registers and return

        CFI_INC_SP(NSPACE)
        CFI_POP2(x21,x30)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_scalarmul)

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_demont_p256)

Lp256_scalarmul_local_demont_p256:
        CFI_START
        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]
        lsl     x7, x2, #32
        subs    x8, x2, x7
        lsr     x6, x2, #32
        sbc     x2, x2, x6
        adds    x3, x3, x7
        adcs    x4, x4, x6
        adcs    x5, x5, x8
        adc     x2, x2, xzr
        lsl     x7, x3, #32
        subs    x8, x3, x7
        lsr     x6, x3, #32
        sbc     x3, x3, x6
        adds    x4, x4, x7
        adcs    x5, x5, x6
        adcs    x2, x2, x8
        adc     x3, x3, xzr
        lsl     x7, x4, #32
        subs    x8, x4, x7
        lsr     x6, x4, #32
        sbc     x4, x4, x6
        adds    x5, x5, x7
        adcs    x2, x2, x6
        adcs    x3, x3, x8
        adc     x4, x4, xzr
        lsl     x7, x5, #32
        subs    x8, x5, x7
        lsr     x6, x5, #32
        sbc     x5, x5, x6
        adds    x2, x2, x7
        adcs    x3, x3, x6
        adcs    x4, x4, x8
        adc     x5, x5, xzr
        stp     x2, x3, [x0]
        stp     x4, x5, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_demont_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_inv_p256)

Lp256_scalarmul_local_inv_p256:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_DEC_SP(160)
        mov     x20, x0
        mov     x10, #0xffffffffffffffff
        mov     x11, #0xffffffff
        mov     x13, #0xffffffff00000001
        stp     x10, x11, [sp]
        stp     xzr, x13, [sp, #16]
        str     xzr, [sp, #32]
        ldp     x2, x3, [x1]
        subs    x10, x2, x10
        sbcs    x11, x3, x11
        ldp     x4, x5, [x1, #16]
        sbcs    x12, x4, xzr
        sbcs    x13, x5, x13
        csel    x2, x2, x10, cc
        csel    x3, x3, x11, cc
        csel    x4, x4, x12, cc
        csel    x5, x5, x13, cc
        stp     x2, x3, [sp, #48]
        stp     x4, x5, [sp, #64]
        str     xzr, [sp, #80]
        stp     xzr, xzr, [sp, #96]
        stp     xzr, xzr, [sp, #112]
        mov     x10, #0x4000000000000
        stp     x10, xzr, [sp, #128]
        stp     xzr, xzr, [sp, #144]
        mov     x21, #0xa
        mov     x22, #0x1
        b       Lp256_scalarmul_inv_midloop
Lp256_scalarmul_inv_loop:
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        and     x0, x12, x16
        and     x1, x13, x17
        add     x19, x0, x1
        ldr     x7, [sp]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #48]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x7, [sp, #8]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #56]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        adc     x6, x6, x1
        extr    x4, x2, x4, #59
        str     x4, [sp]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        adc     x4, x4, x1
        extr    x5, x3, x5, #59
        str     x5, [sp, #48]
        ldr     x7, [sp, #16]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #64]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        adc     x5, x5, x1
        extr    x2, x6, x2, #59
        str     x2, [sp, #8]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        adc     x2, x2, x1
        extr    x3, x4, x3, #59
        str     x3, [sp, #56]
        ldr     x7, [sp, #24]
        eor     x1, x7, x14
        ldr     x23, [sp, #32]
        eor     x3, x23, x14
        and     x3, x3, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #72]
        eor     x1, x8, x15
        ldr     x24, [sp, #80]
        eor     x0, x24, x15
        and     x0, x0, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [sp, #16]
        extr    x5, x3, x5, #59
        str     x5, [sp, #24]
        asr     x3, x3, #59
        str     x3, [sp, #32]
        eor     x1, x7, x16
        eor     x5, x23, x16
        and     x5, x5, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        eor     x0, x24, x17
        and     x0, x0, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x4, x2, x4, #59
        str     x4, [sp, #64]
        extr    x2, x5, x2, #59
        str     x2, [sp, #72]
        asr     x5, x5, #59
        str     x5, [sp, #80]
        ldr     x7, [sp, #96]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #128]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #96]
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        str     x5, [sp, #128]
        adc     x3, x3, x1
        ldr     x7, [sp, #104]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #136]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #104]
        adc     x6, x6, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        str     x3, [sp, #136]
        adc     x4, x4, x1
        ldr     x7, [sp, #112]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #144]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #112]
        adc     x5, x5, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        str     x4, [sp, #144]
        adc     x2, x2, x1
        ldr     x7, [sp, #120]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #152]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldp     x0, x1, [sp, #96]
        ldr     x6, [sp, #112]
        mov     x14, #0xe000000000000000
        adds    x0, x0, x14
        sbcs    x1, x1, xzr
        mov     x11, #0x1fffffff
        adcs    x6, x6, x11
        mov     x10, #0x2000000000000000
        adcs    x5, x5, x10
        mov     x14, #0x1fffffffe0000000
        adc     x3, x3, x14
        lsl     x11, x0, #32
        subs    x14, x0, x11
        lsr     x10, x0, #32
        sbc     x0, x0, x10
        adds    x1, x1, x11
        adcs    x6, x6, x10
        adcs    x5, x5, x14
        adcs    x3, x3, x0
        mov     x14, #0xffffffffffffffff
        mov     x11, #0xffffffff
        mov     x10, #0xffffffff00000001
        csel    x14, x14, xzr, cs
        csel    x11, x11, xzr, cs
        csel    x10, x10, xzr, cs
        subs    x1, x1, x14
        sbcs    x6, x6, x11
        sbcs    x5, x5, xzr
        sbc     x3, x3, x10
        stp     x1, x6, [sp, #96]
        stp     x5, x3, [sp, #112]
        eor     x1, x7, x16
        and     x5, x16, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        and     x0, x17, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        ldp     x0, x1, [sp, #128]
        ldr     x3, [sp, #144]
        mov     x14, #0xe000000000000000
        adds    x0, x0, x14
        sbcs    x1, x1, xzr
        mov     x11, #0x1fffffff
        adcs    x3, x3, x11
        mov     x10, #0x2000000000000000
        adcs    x2, x2, x10
        mov     x14, #0x1fffffffe0000000
        adc     x5, x5, x14
        lsl     x11, x0, #32
        subs    x14, x0, x11
        lsr     x10, x0, #32
        sbc     x0, x0, x10
        adds    x1, x1, x11
        adcs    x3, x3, x10
        adcs    x2, x2, x14
        adcs    x5, x5, x0
        mov     x14, #0xffffffffffffffff
        mov     x11, #0xffffffff
        mov     x10, #0xffffffff00000001
        csel    x14, x14, xzr, cs
        csel    x11, x11, xzr, cs
        csel    x10, x10, xzr, cs
        subs    x1, x1, x14
        sbcs    x3, x3, x11
        sbcs    x2, x2, xzr
        sbc     x5, x5, x10
        stp     x1, x3, [sp, #128]
        stp     x2, x5, [sp, #144]
Lp256_scalarmul_inv_midloop:
        mov     x1, x22
        ldr     x2, [sp]
        ldr     x3, [sp, #48]
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x8, x4, #0x100, lsl #12
        sbfx    x8, x8, #21, #21
        mov     x11, #0x100000
        add     x11, x11, x11, lsl #21
        add     x9, x4, x11
        asr     x9, x9, #42
        add     x10, x5, #0x100, lsl #12
        sbfx    x10, x10, #21, #21
        add     x11, x5, x11
        asr     x11, x11, #42
        mul     x6, x8, x2
        mul     x7, x9, x3
        mul     x2, x10, x2
        mul     x3, x11, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #21, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #42
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #21, #21
        add     x15, x5, x15
        asr     x15, x15, #42
        mul     x6, x12, x2
        mul     x7, x13, x3
        mul     x2, x14, x2
        mul     x3, x15, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        mul     x2, x12, x8
        mul     x3, x12, x9
        mul     x6, x14, x8
        mul     x7, x14, x9
        madd    x8, x13, x10, x2
        madd    x9, x13, x11, x3
        madd    x16, x15, x10, x6
        madd    x17, x15, x11, x7
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #22, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #43
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #22, #21
        add     x15, x5, x15
        asr     x15, x15, #43
        mneg    x2, x12, x8
        mneg    x3, x12, x9
        mneg    x4, x14, x8
        mneg    x5, x14, x9
        msub    x10, x13, x16, x2
        msub    x11, x13, x17, x3
        msub    x12, x15, x16, x4
        msub    x13, x15, x17, x5
        mov     x22, x1
        subs    x21, x21, #0x1
        b.ne    Lp256_scalarmul_inv_loop
        ldr     x0, [sp]
        ldr     x1, [sp, #48]
        mul     x0, x0, x10
        madd    x1, x1, x11, x0
        asr     x0, x1, #63
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        eor     x14, x14, x0
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        eor     x15, x15, x0
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        eor     x16, x16, x0
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        eor     x17, x17, x0
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        ldr     x7, [sp, #96]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #128]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #96]
        adc     x2, x2, x1
        ldr     x7, [sp, #104]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #136]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #104]
        adc     x6, x6, x1
        ldr     x7, [sp, #112]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #144]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #112]
        adc     x5, x5, x1
        ldr     x7, [sp, #120]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #152]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldp     x0, x1, [sp, #96]
        ldr     x2, [sp, #112]
        mov     x14, #0xe000000000000000
        adds    x0, x0, x14
        sbcs    x1, x1, xzr
        mov     x11, #0x1fffffff
        adcs    x2, x2, x11
        mov     x10, #0x2000000000000000
        adcs    x5, x5, x10
        mov     x14, #0x1fffffffe0000000
        adc     x3, x3, x14
        lsl     x11, x0, #32
        subs    x14, x0, x11
        lsr     x10, x0, #32
        sbc     x0, x0, x10
        adds    x1, x1, x11
        adcs    x2, x2, x10
        adcs    x5, x5, x14
        adcs    x3, x3, x0
        mov     x14, #0xffffffffffffffff
        mov     x11, #0xffffffff
        mov     x10, #0xffffffff00000001
        csel    x14, x14, xzr, cs
        csel    x11, x11, xzr, cs
        csel    x10, x10, xzr, cs
        subs    x1, x1, x14
        sbcs    x2, x2, x11
        sbcs    x5, x5, xzr
        sbc     x3, x3, x10
        mov     x10, #0xffffffffffffffff
        subs    x10, x1, x10
        mov     x11, #0xffffffff
        sbcs    x11, x2, x11
        mov     x13, #0xffffffff00000001
        sbcs    x12, x5, xzr
        sbcs    x13, x3, x13
        csel    x10, x1, x10, cc
        csel    x11, x2, x11, cc
        csel    x12, x5, x12, cc
        csel    x13, x3, x13, cc
        stp     x10, x11, [x20]
        stp     x12, x13, [x20, #16]
        CFI_INC_SP(160)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_inv_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_montmul_p256)

Lp256_scalarmul_local_montmul_p256:
        CFI_START
        ldr q20, [x2]
        ldp x7, x17, [x1]
        ldr q0, [x1]
        ldp x6, x10, [x2]
        ldp x11, x15, [x1, #16]
        rev64 v16.4S, v20.4S
        subs x4, x7, x17
        csetm x3, cc
        cneg x13, x4, cc
        mul v16.4S, v16.4S, v0.4S
        umulh x12, x17, x10
        uzp1 v28.4S, v20.4S, v0.4S
        subs x14, x11, x7
        ldr q20, [x2, #16]
        sbcs x5, x15, x17
        ngc x17, xzr
        subs x8, x11, x15
        uaddlp v27.2D, v16.4S
        umulh x4, x7, x6
        uzp1 v21.4S, v0.4S, v0.4S
        cneg x11, x8, cc
        shl v17.2D, v27.2D, #32
        csetm x15, cc
        subs x9, x10, x6
        eor x7, x14, x17
        umlal v17.2D, v21.2S, v28.2S
        cneg x8, x9, cc
        cinv x9, x3, cc
        cmn x17, #0x1
        ldr q28, [x1, #16]
        adcs x14, x7, xzr
        mul x7, x13, x8
        eor x1, x5, x17
        adcs x5, x1, xzr
        xtn v1.2S, v20.2D
        mov x1, v17.d[0]
        mov x3, v17.d[1]
        uzp2 v16.4S, v20.4S, v20.4S
        umulh x16, x13, x8
        eor x13, x7, x9
        adds x8, x1, x3
        adcs x7, x4, x12
        xtn v0.2S, v28.2D
        adcs x12, x12, xzr
        adds x8, x4, x8
        adcs x3, x3, x7
        ldp x7, x2, [x2, #16]
        adcs x12, x12, xzr
        cmn x9, #0x1
        adcs x8, x8, x13
        eor x13, x16, x9
        adcs x16, x3, x13
        lsl x3, x1, #32
        adc x13, x12, x9
        subs x12, x6, x7
        sbcs x9, x10, x2
        lsr x10, x1, #32
        ngc x4, xzr
        subs x6, x2, x7
        cinv x2, x15, cc
        cneg x6, x6, cc
        subs x7, x1, x3
        eor x9, x9, x4
        sbc x1, x1, x10
        adds x15, x8, x3
        adcs x3, x16, x10
        mul x16, x11, x6
        adcs x8, x13, x7
        eor x13, x12, x4
        adc x10, x1, xzr
        cmn x4, #0x1
        umulh x6, x11, x6
        adcs x11, x13, xzr
        adcs x1, x9, xzr
        lsl x13, x15, #32
        subs x12, x15, x13
        lsr x7, x15, #32
        sbc x15, x15, x7
        adds x9, x3, x13
        adcs x3, x8, x7
        umulh x8, x14, x11
        umull v21.2D, v0.2S, v1.2S
        adcs x12, x10, x12
        umull v3.2D, v0.2S, v16.2S
        adc x15, x15, xzr
        rev64 v24.4S, v20.4S
        stp x12, x15, [x0, #16]
        movi v2.2D, #0x00000000ffffffff
        mul x10, x14, x11
        mul v4.4S, v24.4S, v28.4S
        subs x13, x14, x5
        uzp2 v19.4S, v28.4S, v28.4S
        csetm x15, cc
        usra v3.2D, v21.2D, #32
        mul x7, x5, x1
        umull v21.2D, v19.2S, v16.2S
        cneg x13, x13, cc
        uaddlp v5.2D, v4.4S
        subs x11, x1, x11
        and v16.16B, v3.16B, v2.16B
        umulh x5, x5, x1
        shl v24.2D, v5.2D, #32
        cneg x11, x11, cc
        umlal v16.2D, v19.2S, v1.2S
        cinv x12, x15, cc
        umlal v24.2D, v0.2S, v1.2S
        adds x15, x10, x7
        mul x14, x13, x11
        eor x1, x6, x2
        adcs x6, x8, x5
        stp x9, x3, [x0]
        usra v21.2D, v3.2D, #32
        adcs x9, x5, xzr
        umulh x11, x13, x11
        adds x15, x8, x15
        adcs x7, x7, x6
        eor x8, x14, x12
        usra v21.2D, v16.2D, #32
        adcs x13, x9, xzr
        cmn x12, #0x1
        mov x9, v24.d[1]
        adcs x14, x15, x8
        eor x6, x11, x12
        adcs x6, x7, x6
        mov x5, v24.d[0]
        mov x11, v21.d[1]
        mov x7, v21.d[0]
        adc x3, x13, x12
        adds x12, x5, x9
        adcs x13, x7, x11
        ldp x15, x8, [x0]
        adcs x11, x11, xzr
        adds x12, x7, x12
        eor x16, x16, x2
        adcs x7, x9, x13
        adcs x11, x11, xzr
        cmn x2, #0x1
        ldp x9, x13, [x0, #16]
        adcs x16, x12, x16
        adcs x1, x7, x1
        adc x2, x11, x2
        adds x7, x5, x15
        adcs x15, x16, x8
        eor x5, x17, x4
        adcs x9, x1, x9
        eor x1, x10, x5
        adcs x16, x2, x13
        adc x2, xzr, xzr
        cmn x5, #0x1
        eor x13, x14, x5
        adcs x14, x1, x7
        eor x1, x6, x5
        adcs x6, x13, x15
        adcs x10, x1, x9
        eor x4, x3, x5
        mov x1, #0xffffffff
        adcs x8, x4, x16
        lsr x13, x14, #32
        adcs x17, x2, x5
        adcs x11, x5, xzr
        adc x4, x5, xzr
        adds x12, x10, x7
        adcs x7, x8, x15
        adcs x5, x17, x9
        adcs x9, x11, x16
        lsl x11, x14, #32
        adc x10, x4, x2
        subs x17, x14, x11
        sbc x4, x14, x13
        adds x11, x6, x11
        adcs x12, x12, x13
        lsl x15, x11, #32
        adcs x17, x7, x17
        lsr x7, x11, #32
        adc x13, x4, xzr
        subs x4, x11, x15
        sbc x11, x11, x7
        adds x8, x12, x15
        adcs x15, x17, x7
        adcs x4, x13, x4
        adc x11, x11, xzr
        adds x7, x5, x4
        adcs x17, x9, x11
        adc x13, x10, xzr
        add x12, x13, #0x1
        neg x11, x12
        lsl x4, x12, #32
        adds x17, x17, x4
        sub x4, x4, #0x1
        adc x13, x13, xzr
        subs x11, x8, x11
        sbcs x4, x15, x4
        sbcs x7, x7, xzr
        sbcs x17, x17, x12
        sbcs x13, x13, x12
        mov x12, #0xffffffff00000001
        adds x11, x11, x13
        and x1, x1, x13
        adcs x4, x4, x1
        and x1, x12, x13
        stp x11, x4, [x0]
        adcs x4, x7, xzr
        adc x1, x17, x1
        stp x4, x1, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_montmul_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_montsqr_p256)

Lp256_scalarmul_local_montsqr_p256:
        CFI_START
        ldr q19, [x1]
        ldp x9, x13, [x1]
        ldr q23, [x1, #16]
        ldr q0, [x1]
        ldp x1, x10, [x1, #16]
        uzp2 v29.4S, v19.4S, v19.4S
        xtn v4.2S, v19.2D
        umulh x8, x9, x13
        rev64 v20.4S, v23.4S
        umull v16.2D, v19.2S, v19.2S
        umull v1.2D, v29.2S, v4.2S
        mul v20.4S, v20.4S, v0.4S
        subs x14, x9, x13
        umulh x15, x9, x1
        mov x16, v16.d[1]
        umull2 v4.2D, v19.4S, v19.4S
        mov x4, v16.d[0]
        uzp1 v17.4S, v23.4S, v0.4S
        uaddlp v19.2D, v20.4S
        lsr x7, x8, #63
        mul x11, x9, x13
        mov x12, v1.d[0]
        csetm x5, cc
        cneg x6, x14, cc
        mov x3, v4.d[1]
        mov x14, v4.d[0]
        subs x2, x10, x1
        mov x9, v1.d[1]
        cneg x17, x2, cc
        cinv x2, x5, cc
        adds x5, x4, x12, lsl #33
        extr x4, x8, x11, #63
        lsr x8, x12, #31
        uzp1 v20.4S, v0.4S, v0.4S
        shl v19.2D, v19.2D, #32
        adc x16, x16, x8
        adds x8, x14, x9, lsl #33
        lsr x14, x9, #31
        lsl x9, x5, #32
        umlal v19.2D, v20.2S, v17.2S
        adc x14, x3, x14
        adds x16, x16, x11, lsl #1
        lsr x3, x5, #32
        umulh x12, x6, x17
        adcs x4, x8, x4
        adc x11, x14, x7
        subs x8, x5, x9
        sbc x5, x5, x3
        adds x16, x16, x9
        mov x14, v19.d[0]
        mul x17, x6, x17
        adcs x3, x4, x3
        lsl x7, x16, #32
        umulh x13, x13, x10
        adcs x11, x11, x8
        lsr x8, x16, #32
        adc x5, x5, xzr
        subs x9, x16, x7
        sbc x16, x16, x8
        adds x7, x3, x7
        mov x3, v19.d[1]
        adcs x6, x11, x8
        umulh x11, x1, x10
        adcs x5, x5, x9
        eor x8, x12, x2
        adc x9, x16, xzr
        adds x16, x14, x15
        adc x15, x15, xzr
        adds x12, x16, x3
        eor x16, x17, x2
        mul x4, x1, x10
        adcs x15, x15, x13
        adc x17, x13, xzr
        adds x15, x15, x3
        adc x3, x17, xzr
        cmn x2, #0x1
        mul x17, x10, x10
        adcs x12, x12, x16
        adcs x16, x15, x8
        umulh x10, x10, x10
        adc x2, x3, x2
        adds x14, x14, x14
        adcs x12, x12, x12
        adcs x16, x16, x16
        adcs x2, x2, x2
        adc x15, xzr, xzr
        adds x14, x14, x7
        mul x3, x1, x1
        adcs x12, x12, x6
        lsr x7, x14, #32
        adcs x16, x16, x5
        lsl x5, x14, #32
        umulh x13, x1, x1
        adcs x2, x2, x9
        mov x6, #0xffffffff
        adc x15, x15, xzr
        adds x8, x4, x4
        adcs x1, x11, x11
        mov x11, #0xffffffff00000001
        adc x4, xzr, xzr
        subs x9, x14, x5
        sbc x14, x14, x7
        adds x12, x12, x5
        adcs x16, x16, x7
        lsl x5, x12, #32
        lsr x7, x12, #32
        adcs x2, x2, x9
        adcs x14, x15, x14
        adc x15, xzr, xzr
        subs x9, x12, x5
        sbc x12, x12, x7
        adds x16, x16, x5
        adcs x2, x2, x7
        adcs x14, x14, x9
        adcs x12, x15, x12
        adc x15, xzr, xzr
        adds x16, x16, x3
        adcs x2, x2, x13
        adcs x14, x14, x17
        adcs x12, x12, x10
        adc x15, x15, xzr
        adds x2, x2, x8
        adcs x14, x14, x1
        adcs x12, x12, x4
        adcs x15, x15, xzr
        adds x3, x16, #0x1
        sbcs x5, x2, x6
        sbcs x8, x14, xzr
        sbcs x11, x12, x11
        sbcs xzr, x15, xzr
        csel x16, x3, x16, cs
        csel x14, x8, x14, cs
        csel x12, x11, x12, cs
        csel x2, x5, x2, cs
        stp x14, x12, [x0, #16]
        stp x16, x2, [x0]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_montsqr_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_tomont_p256)

Lp256_scalarmul_local_tomont_p256:
        CFI_START
        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]
        mov     x1, #0xffffffffffffffff
        mov     x7, #0xffffffff
        mov     x9, #0xffffffff00000001
        subs    x1, x2, x1
        sbcs    x7, x3, x7
        sbcs    x8, x4, xzr
        sbcs    x9, x5, x9
        csel    x2, x2, x1, cc
        csel    x3, x3, x7, cc
        csel    x4, x4, x8, cc
        csel    x5, x5, x9, cc
        cmp     xzr, xzr
        extr    x9, x5, x4, #32
        adcs    xzr, x4, x9
        lsr     x9, x5, #32
        adcs    x9, x5, x9
        csetm   x6, cs
        orr     x9, x9, x6
        lsl     x7, x9, #32
        lsr     x8, x9, #32
        adds    x4, x4, x7
        adc     x5, x5, x8
        negs    x6, x9
        sbcs    x7, x7, xzr
        sbc     x8, x8, xzr
        negs    x6, x6
        sbcs    x2, x2, x7
        sbcs    x3, x3, x8
        sbcs    x4, x4, x9
        sbcs    x5, x5, x9
        adds    x6, x6, x5
        mov     x7, #0xffffffff
        and     x7, x7, x5
        adcs    x2, x2, x7
        adcs    x3, x3, xzr
        mov     x7, #0xffffffff00000001
        and     x7, x7, x5
        adc     x4, x4, x7
        cmp     xzr, xzr
        extr    x9, x4, x3, #32
        adcs    xzr, x3, x9
        lsr     x9, x4, #32
        adcs    x9, x4, x9
        csetm   x5, cs
        orr     x9, x9, x5
        lsl     x7, x9, #32
        lsr     x8, x9, #32
        adds    x3, x3, x7
        adc     x4, x4, x8
        negs    x5, x9
        sbcs    x7, x7, xzr
        sbc     x8, x8, xzr
        negs    x5, x5
        sbcs    x6, x6, x7
        sbcs    x2, x2, x8
        sbcs    x3, x3, x9
        sbcs    x4, x4, x9
        adds    x5, x5, x4
        mov     x7, #0xffffffff
        and     x7, x7, x4
        adcs    x6, x6, x7
        adcs    x2, x2, xzr
        mov     x7, #0xffffffff00000001
        and     x7, x7, x4
        adc     x3, x3, x7
        cmp     xzr, xzr
        extr    x9, x3, x2, #32
        adcs    xzr, x2, x9
        lsr     x9, x3, #32
        adcs    x9, x3, x9
        csetm   x4, cs
        orr     x9, x9, x4
        lsl     x7, x9, #32
        lsr     x8, x9, #32
        adds    x2, x2, x7
        adc     x3, x3, x8
        negs    x4, x9
        sbcs    x7, x7, xzr
        sbc     x8, x8, xzr
        negs    x4, x4
        sbcs    x5, x5, x7
        sbcs    x6, x6, x8
        sbcs    x2, x2, x9
        sbcs    x3, x3, x9
        adds    x4, x4, x3
        mov     x7, #0xffffffff
        and     x7, x7, x3
        adcs    x5, x5, x7
        adcs    x6, x6, xzr
        mov     x7, #0xffffffff00000001
        and     x7, x7, x3
        adc     x2, x2, x7
        cmp     xzr, xzr
        extr    x9, x2, x6, #32
        adcs    xzr, x6, x9
        lsr     x9, x2, #32
        adcs    x9, x2, x9
        csetm   x3, cs
        orr     x9, x9, x3
        lsl     x7, x9, #32
        lsr     x8, x9, #32
        adds    x6, x6, x7
        adc     x2, x2, x8
        negs    x3, x9
        sbcs    x7, x7, xzr
        sbc     x8, x8, xzr
        negs    x3, x3
        sbcs    x4, x4, x7
        sbcs    x5, x5, x8
        sbcs    x6, x6, x9
        sbcs    x2, x2, x9
        adds    x3, x3, x2
        mov     x7, #0xffffffff
        and     x7, x7, x2
        adcs    x4, x4, x7
        adcs    x5, x5, xzr
        mov     x7, #0xffffffff00000001
        and     x7, x7, x2
        adc     x6, x6, x7
        stp     x3, x4, [x0]
        stp     x5, x6, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_tomont_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_p256_montjadd)

Lp256_scalarmul_local_p256_montjadd:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x30)
        CFI_DEC_SP(224)
        mov     x21, x0
        mov     x22, x1
        mov     x23, x2
        mov     x0, sp
        ldr     q19, [x22, #64]
        ldp     x9, x13, [x22, #64]
        ldr     q23, [x22, #80]
        ldr     q0, [x22, #64]
        ldp     x1, x10, [x22, #80]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x9, x13
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x9, x13
        umulh   x15, x9, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x9, x13
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x13, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x19, x3, x16, cs  // cs = hs, nlast
        csel    x14, x8, x14, cs  // cs = hs, nlast
        csel    x12, x11, x12, cs  // cs = hs, nlast
        csel    x20, x5, x2, cs  // cs = hs, nlast
        stp     x14, x12, [x0, #16]
        stp     x19, x20, [x0]
        ldr     q19, [x23, #64]
        ldp     x9, x13, [x23, #64]
        ldr     q23, [x23, #80]
        ldr     q0, [x23, #64]
        ldp     x1, x10, [x23, #80]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x9, x13
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x9, x13
        umulh   x15, x9, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x9, x13
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x13, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x16, x3, x16, cs  // cs = hs, nlast
        csel    x14, x8, x14, cs  // cs = hs, nlast
        csel    x12, x11, x12, cs  // cs = hs, nlast
        csel    x2, x5, x2, cs  // cs = hs, nlast
        stp     x14, x12, [sp, #176]
        stp     x16, x2, [sp, #160]
        ldr     q20, [x22, #32]
        ldp     x7, x17, [x23, #64]
        ldr     q0, [x23, #64]
        ldp     x6, x10, [x22, #32]
        ldp     x11, x15, [x23, #80]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [x22, #48]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [x23, #80]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [x22, #48]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x24, x3, x13
        adcs    x25, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x24
        adcs    x15, x16, x25
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x11, x11, x13
        and     x1, x1, x13
        adcs    x4, x4, x1
        and     x1, x12, x13
        stp     x11, x4, [sp, #192]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #208]
        ldr     q20, [x23, #32]
        ldp     x7, x17, [x22, #64]
        ldr     q0, [x22, #64]
        ldp     x6, x10, [x23, #32]
        ldp     x11, x15, [x22, #80]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [x23, #48]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [x22, #80]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [x23, #48]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x24, x3, x13
        adcs    x25, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x24
        adcs    x15, x16, x25
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x24, x11, x13
        and     x1, x1, x13
        adcs    x25, x4, x1
        and     x1, x12, x13
        stp     x24, x25, [sp, #32]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #48]
        mov     x1, sp
        ldr     q20, [x23]
        ldr     q0, [x1]
        ldp     x6, x10, [x23]
        ldp     x11, x15, [x1, #16]
        rev64   v16.4s, v20.4s
        subs    x4, x19, x20
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x20, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x19
        ldr     q20, [x23, #16]
        sbcs    x5, x15, x20
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x19, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [x1, #16]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [x23, #16]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x19, x3, x13
        adcs    x20, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x19
        adcs    x15, x16, x20
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x11, x11, x13
        and     x1, x1, x13
        adcs    x4, x4, x1
        and     x1, x12, x13
        stp     x11, x4, [sp, #64]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #80]
        ldr     q20, [x22]
        ldp     x7, x17, [sp, #160]
        ldr     q0, [sp, #160]
        ldp     x6, x10, [x22]
        ldp     x11, x15, [sp, #176]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [x22, #16]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #176]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [x22, #16]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x19, x3, x13
        adcs    x20, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x19
        adcs    x15, x16, x20
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x11, x11, x13
        and     x1, x1, x13
        adcs    x4, x4, x1
        and     x1, x12, x13
        stp     x11, x4, [sp, #128]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #144]
        mov     x1, sp
        ldr     q20, [sp, #32]
        ldp     x7, x17, [x1]
        ldr     q0, [x1]
        ldp     x11, x15, [x1, #16]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x25
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [sp, #48]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x24
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x25, x24
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [x1, #16]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #48]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x24, x7
        sbcs    x9, x25, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x19, x3, x13
        adcs    x20, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x24, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x25, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x19
        adcs    x15, x16, x20
        eor     x5, x17, x4
        adcs    x9, x1, x24
        eor     x1, x10, x5
        adcs    x16, x2, x25
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x19, x11, x13
        and     x1, x1, x13
        adcs    x20, x4, x1
        and     x1, x12, x13
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #48]
        ldr     q20, [sp, #192]
        ldp     x7, x17, [sp, #160]
        ldr     q0, [sp, #160]
        ldp     x6, x10, [sp, #192]
        ldp     x11, x15, [sp, #176]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [sp, #208]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #176]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #208]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x24, x3, x13
        adcs    x25, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x24
        adcs    x15, x16, x25
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x9, x11, x13
        and     x1, x1, x13
        adcs    x10, x4, x1
        and     x1, x12, x13
        stp     x9, x10, [sp, #192]
        adcs    x11, x7, xzr
        adc     x12, x17, x1
        stp     x11, x12, [sp, #208]
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [sp, #128]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [sp, #144]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x13, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x24, x6, x4
        adcs    x25, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x26, x8, x4
        stp     x13, x24, [sp, #160]
        stp     x25, x26, [sp, #176]
        subs    x5, x19, x9
        sbcs    x6, x20, x10
        ldp     x7, x8, [sp, #48]
        sbcs    x7, x7, x11
        sbcs    x8, x8, x12
        csetm   x3, cc  // cc = lo, ul, last
        adds    x19, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x20, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x19, x20, [sp, #32]
        stp     x7, x8, [sp, #48]
        ldr     q19, [sp, #160]
        ldr     q23, [sp, #176]
        ldr     q0, [sp, #160]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x13, x24
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x13, x24
        umulh   x15, x13, x25
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x13, x24
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x26, x25
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x24, x26
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x25, x26
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x25, x26
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x26, x26
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x26, x26
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x25, x25
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x25, x25
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x24, x3, x16, cs  // cs = hs, nlast
        csel    x25, x8, x14, cs  // cs = hs, nlast
        csel    x26, x11, x12, cs  // cs = hs, nlast
        csel    x27, x5, x2, cs  // cs = hs, nlast
        stp     x25, x26, [sp, #112]
        stp     x24, x27, [sp, #96]
        mov     x0, sp
        ldr     q19, [sp, #32]
        ldr     q23, [sp, #48]
        ldr     q0, [sp, #32]
        ldp     x1, x10, [sp, #48]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x19, x20
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x19, x20
        umulh   x15, x19, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x19, x20
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x20, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x16, x3, x16, cs  // cs = hs, nlast
        csel    x14, x8, x14, cs  // cs = hs, nlast
        csel    x12, x11, x12, cs  // cs = hs, nlast
        csel    x2, x5, x2, cs  // cs = hs, nlast
        stp     x14, x12, [x0, #16]
        stp     x16, x2, [x0]
        ldr     q20, [sp, #128]
        ldr     q0, [sp, #96]
        ldp     x6, x10, [sp, #128]
        rev64   v16.4s, v20.4s
        subs    x4, x24, x27
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x27, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x25, x24
        ldr     q20, [sp, #144]
        sbcs    x5, x26, x27
        ngc     x17, xzr
        subs    x8, x25, x26
        uaddlp  v27.2d, v16.4s
        umulh   x4, x24, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #112]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #144]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x19, x3, x13
        adcs    x20, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x25, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x26, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x19
        adcs    x15, x16, x20
        eor     x5, x17, x4
        adcs    x9, x1, x25
        eor     x1, x10, x5
        adcs    x16, x2, x26
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x19, x11, x13
        and     x1, x1, x13
        adcs    x20, x4, x1
        and     x1, x12, x13
        stp     x19, x20, [sp, #128]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #144]
        ldr     q20, [sp, #64]
        ldr     q0, [sp, #96]
        ldp     x6, x10, [sp, #64]
        ldp     x11, x15, [sp, #112]
        rev64   v16.4s, v20.4s
        subs    x4, x24, x27
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x27, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x24
        ldr     q20, [sp, #80]
        sbcs    x5, x15, x27
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x24, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #112]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #80]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x24, x3, x13
        adcs    x25, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x24
        adcs    x15, x16, x25
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x9, x11, x13
        and     x1, x1, x13
        adcs    x10, x4, x1
        and     x1, x12, x13
        stp     x9, x10, [sp, #64]
        adcs    x11, x7, xzr
        adc     x12, x17, x1
        stp     x11, x12, [sp, #80]
        mov     x0, sp
        mov     x1, sp
        ldp     x5, x6, [x1]
        subs    x5, x5, x19
        sbcs    x6, x6, x20
        ldp     x7, x8, [x1, #16]
        ldp     x4, x3, [sp, #144]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x24, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x25, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x7, x8, [x0, #16]
        subs    x5, x9, x19
        sbcs    x6, x10, x20
        ldp     x4, x3, [sp, #144]
        sbcs    x7, x11, x4
        sbcs    x8, x12, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [sp, #96]
        stp     x7, x8, [sp, #112]
        ldr     q20, [x22, #64]
        ldp     x7, x17, [sp, #160]
        ldr     q0, [sp, #160]
        ldp     x6, x10, [x22, #64]
        ldp     x11, x15, [sp, #176]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [x22, #80]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #176]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [x22, #80]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x19, x3, x13
        adcs    x20, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x19
        adcs    x15, x16, x20
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x11, x11, x13
        and     x1, x1, x13
        adcs    x4, x4, x1
        and     x1, x12, x13
        stp     x11, x4, [sp, #160]
        adcs    x19, x7, xzr
        adc     x20, x17, x1
        stp     x19, x20, [sp, #176]
        mov     x0, sp
        mov     x1, sp
        ldp     x4, x3, [sp, #64]
        subs    x5, x24, x4
        sbcs    x6, x25, x3
        ldp     x7, x8, [x1, #16]
        ldp     x4, x3, [sp, #80]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x9, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x10, x6, x4
        adcs    x11, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x3, x8, x4
        stp     x9, x10, [x0]
        stp     x11, x3, [x0, #16]
        ldp     x5, x6, [sp, #128]
        subs    x5, x5, x9
        sbcs    x6, x6, x10
        ldp     x7, x8, [sp, #144]
        sbcs    x7, x7, x11
        sbcs    x8, x8, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [sp, #128]
        stp     x7, x8, [sp, #144]
        ldr     q20, [sp, #192]
        ldp     x7, x17, [sp, #96]
        ldr     q0, [sp, #96]
        ldp     x6, x10, [sp, #192]
        ldp     x11, x15, [sp, #112]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [sp, #208]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #112]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #208]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x24, x3, x13
        adcs    x25, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x24
        adcs    x15, x16, x25
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x11, x11, x13
        and     x1, x1, x13
        adcs    x4, x4, x1
        and     x1, x12, x13
        stp     x11, x4, [sp, #96]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #112]
        ldr     q20, [x23, #64]
        ldp     x7, x17, [sp, #160]
        ldr     q0, [sp, #160]
        ldp     x6, x10, [x23, #64]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x19, x7
        ldr     q20, [x23, #80]
        sbcs    x5, x20, x17
        ngc     x17, xzr
        subs    x8, x19, x20
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #176]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [x23, #80]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x19, x3, x13
        adcs    x20, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x24, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x25, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x19
        adcs    x15, x16, x20
        eor     x5, x17, x4
        adcs    x9, x1, x24
        eor     x1, x10, x5
        adcs    x16, x2, x25
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x19, x11, x13
        and     x1, x1, x13
        adcs    x20, x4, x1
        and     x1, x12, x13
        stp     x19, x20, [sp, #160]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #176]
        ldr     q20, [sp, #128]
        ldp     x7, x17, [sp, #32]
        ldr     q0, [sp, #32]
        ldp     x6, x10, [sp, #128]
        ldp     x11, x15, [sp, #48]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [sp, #144]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #48]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #144]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x24, x3, x13
        adcs    x25, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x24
        adcs    x15, x16, x25
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x5, x11, x13
        and     x1, x1, x13
        adcs    x6, x4, x1
        and     x1, x12, x13
        adcs    x7, x7, xzr
        adc     x9, x17, x1
        ldp     x4, x3, [sp, #96]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x4, x3, [sp, #112]
        sbcs    x7, x7, x4
        sbcs    x8, x9, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x15, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x24, x6, x4
        adcs    x25, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x26, x8, x4
        stp     x15, x24, [sp, #128]
        stp     x25, x26, [sp, #144]
        ldp     x0, x1, [x22, #64]
        ldp     x2, x3, [x22, #80]
        orr     x12, x0, x1
        orr     x13, x2, x3
        orr     x12, x12, x13
        cmp     x12, xzr
        cset    x12, ne  // ne = any
        ldp     x4, x5, [x23, #64]
        ldp     x6, x7, [x23, #80]
        orr     x13, x4, x5
        orr     x14, x6, x7
        orr     x13, x13, x14
        cmp     x13, xzr
        cset    x13, ne  // ne = any
        cmp     x13, x12
        csel    x8, x0, x19, cc  // cc = lo, ul, last
        csel    x9, x1, x20, cc  // cc = lo, ul, last
        csel    x8, x4, x8, hi  // hi = pmore
        csel    x9, x5, x9, hi  // hi = pmore
        ldp     x10, x11, [sp, #176]
        csel    x10, x2, x10, cc  // cc = lo, ul, last
        csel    x11, x3, x11, cc  // cc = lo, ul, last
        csel    x10, x6, x10, hi  // hi = pmore
        csel    x11, x7, x11, hi  // hi = pmore
        ldp     x12, x13, [x22]
        ldp     x0, x1, [sp]
        csel    x0, x12, x0, cc  // cc = lo, ul, last
        csel    x1, x13, x1, cc  // cc = lo, ul, last
        ldp     x12, x13, [x23]
        csel    x0, x12, x0, hi  // hi = pmore
        csel    x1, x13, x1, hi  // hi = pmore
        ldp     x12, x13, [x22, #16]
        ldp     x2, x3, [sp, #16]
        csel    x2, x12, x2, cc  // cc = lo, ul, last
        csel    x3, x13, x3, cc  // cc = lo, ul, last
        ldp     x12, x13, [x23, #16]
        csel    x2, x12, x2, hi  // hi = pmore
        csel    x3, x13, x3, hi  // hi = pmore
        ldp     x12, x13, [x22, #32]
        csel    x4, x12, x15, cc  // cc = lo, ul, last
        csel    x5, x13, x24, cc  // cc = lo, ul, last
        ldp     x12, x13, [x23, #32]
        csel    x4, x12, x4, hi  // hi = pmore
        csel    x5, x13, x5, hi  // hi = pmore
        ldp     x12, x13, [x22, #48]
        csel    x6, x12, x25, cc  // cc = lo, ul, last
        csel    x7, x13, x26, cc  // cc = lo, ul, last
        ldp     x12, x13, [x23, #48]
        csel    x6, x12, x6, hi  // hi = pmore
        csel    x7, x13, x7, hi  // hi = pmore
        stp     x0, x1, [x21]
        stp     x2, x3, [x21, #16]
        stp     x4, x5, [x21, #32]
        stp     x6, x7, [x21, #48]
        stp     x8, x9, [x21, #64]
        stp     x10, x11, [x21, #80]
        CFI_INC_SP(224)
        CFI_POP2(x27,x30)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_p256_montjadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_p256_montjdouble)

Lp256_scalarmul_local_p256_montjdouble:
        CFI_START
        CFI_DEC_SP(272)
        stp     x19, x20, [sp, #192]
        stp     x21, x22, [sp, #208]
        stp     x23, x24, [sp, #224]
        stp     x25, x26, [sp, #240]
        stp     x27, xzr, [sp, #256]
        mov     x19, x0
        mov     x20, x1
        mov     x0, sp
        ldr     q19, [x20, #64]
        ldp     x9, x13, [x20, #64]
        ldr     q23, [x20, #80]
        ldr     q0, [x20, #64]
        ldp     x1, x10, [x20, #80]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x9, x13
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x9, x13
        umulh   x15, x9, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x9, x13
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x13, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x21, x3, x16, cs  // cs = hs, nlast
        csel    x22, x8, x14, cs  // cs = hs, nlast
        csel    x23, x11, x12, cs  // cs = hs, nlast
        csel    x24, x5, x2, cs  // cs = hs, nlast
        stp     x22, x23, [x0, #16]
        stp     x21, x24, [x0]
        ldr     q19, [x20, #32]
        ldp     x9, x13, [x20, #32]
        ldr     q23, [x20, #48]
        ldr     q0, [x20, #32]
        ldp     x1, x10, [x20, #48]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x9, x13
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x9, x13
        umulh   x15, x9, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x9, x13
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x13, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x16, x3, x16, cs  // cs = hs, nlast
        csel    x14, x8, x14, cs  // cs = hs, nlast
        csel    x12, x11, x12, cs  // cs = hs, nlast
        csel    x2, x5, x2, cs  // cs = hs, nlast
        stp     x14, x12, [sp, #48]
        stp     x16, x2, [sp, #32]
        ldp     x5, x6, [x20]
        subs    x5, x5, x21
        sbcs    x6, x6, x24
        ldp     x7, x8, [x20, #16]
        sbcs    x7, x7, x22
        sbcs    x8, x8, x23
        csetm   x3, cc  // cc = lo, ul, last
        adds    x10, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x25, x6, x4
        adcs    x26, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x27, x8, x4
        stp     x10, x25, [sp, #96]
        stp     x26, x27, [sp, #112]
        ldp     x5, x6, [x20]
        adds    x5, x5, x21
        adcs    x6, x6, x24
        ldp     x7, x8, [x20, #16]
        adcs    x7, x7, x22
        adcs    x8, x8, x23
        csetm   x3, cs  // cs = hs, nlast
        subs    x9, x5, x3
        and     x1, x3, #0xffffffff
        sbcs    x5, x6, x1
        sbcs    x7, x7, xzr
        and     x2, x3, #0xffffffff00000001
        sbc     x8, x8, x2
        stp     x9, x5, [sp, #64]
        stp     x7, x8, [sp, #80]
        ldr     q20, [sp, #96]
        ldr     q0, [sp, #64]
        rev64   v16.4s, v20.4s
        subs    x4, x9, x5
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x5, x25
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x7, x9
        ldr     q20, [sp, #112]
        sbcs    x5, x8, x5
        ngc     x17, xzr
        subs    x8, x7, x8
        uaddlp  v27.2d, v16.4s
        umulh   x4, x9, x10
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x25, x10
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #80]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x10, x26
        sbcs    x9, x25, x27
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x27, x26
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x21, x3, x13
        adcs    x22, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x23, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x24, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x21
        adcs    x15, x16, x22
        eor     x5, x17, x4
        adcs    x9, x1, x23
        eor     x1, x10, x5
        adcs    x16, x2, x24
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x21, x11, x13
        and     x1, x1, x13
        adcs    x22, x4, x1
        and     x1, x12, x13
        stp     x21, x22, [sp, #96]
        adcs    x23, x7, xzr
        adc     x24, x17, x1
        stp     x23, x24, [sp, #112]
        ldp     x4, x5, [x20, #32]
        ldp     x8, x9, [x20, #64]
        adds    x4, x4, x8
        adcs    x5, x5, x9
        ldp     x6, x7, [x20, #48]
        ldp     x10, x11, [x20, #80]
        adcs    x6, x6, x10
        adcs    x7, x7, x11
        adc     x3, xzr, xzr
        adds    x8, x4, #0x1
        mov     x9, #0xffffffff                 // #4294967295
        sbcs    x9, x5, x9
        sbcs    x10, x6, xzr
        mov     x11, #0xffffffff00000001        // #-4294967295
        sbcs    x11, x7, x11
        sbcs    x3, x3, xzr
        csel    x4, x4, x8, cc  // cc = lo, ul, last
        csel    x5, x5, x9, cc  // cc = lo, ul, last
        csel    x6, x6, x10, cc  // cc = lo, ul, last
        csel    x7, x7, x11, cc  // cc = lo, ul, last
        stp     x4, x5, [sp, #64]
        stp     x6, x7, [sp, #80]
        ldr     q20, [sp, #32]
        ldp     x7, x17, [x20]
        ldr     q0, [x20]
        ldp     x6, x10, [sp, #32]
        ldp     x11, x15, [x20, #16]
        rev64   v16.4s, v20.4s
        subs    x4, x7, x17
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x17, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x11, x7
        ldr     q20, [sp, #48]
        sbcs    x5, x15, x17
        ngc     x17, xzr
        subs    x8, x11, x15
        uaddlp  v27.2d, v16.4s
        umulh   x4, x7, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [x20, #16]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #48]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x20, x3, x13
        adcs    x25, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x26, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x27, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x20
        adcs    x15, x16, x25
        eor     x5, x17, x4
        adcs    x9, x1, x26
        eor     x1, x10, x5
        adcs    x16, x2, x27
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x20, x11, x13
        and     x1, x1, x13
        adcs    x25, x4, x1
        and     x1, x12, x13
        stp     x20, x25, [sp, #128]
        adcs    x4, x7, xzr
        adc     x1, x17, x1
        stp     x4, x1, [sp, #144]
        ldr     q19, [sp, #96]
        ldr     q23, [sp, #112]
        ldr     q0, [sp, #96]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x21, x22
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x21, x22
        umulh   x15, x21, x23
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x21, x22
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x24, x23
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x22, x24
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x23, x24
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x23, x24
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x24, x24
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x24, x24
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x23, x23
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x23, x23
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x21, x3, x16, cs  // cs = hs, nlast
        csel    x22, x8, x14, cs  // cs = hs, nlast
        csel    x23, x11, x12, cs  // cs = hs, nlast
        csel    x24, x5, x2, cs  // cs = hs, nlast
        ldr     q19, [sp, #64]
        ldp     x9, x13, [sp, #64]
        ldr     q23, [sp, #80]
        ldr     q0, [sp, #64]
        ldp     x1, x10, [sp, #80]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x9, x13
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x9, x13
        umulh   x15, x9, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x9, x13
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x13, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x13, x3, x16, cs  // cs = hs, nlast
        csel    x14, x8, x14, cs  // cs = hs, nlast
        csel    x15, x11, x12, cs  // cs = hs, nlast
        csel    x26, x5, x2, cs  // cs = hs, nlast
        mov     x1, #0x9                        // #9
        mov     x2, #0xffffffffffffffff         // #-1
        subs    x9, x2, x21
        mov     x2, #0xffffffff                 // #4294967295
        sbcs    x10, x2, x24
        ngcs    x11, x22
        mov     x2, #0xffffffff00000001         // #-4294967295
        sbc     x12, x2, x23
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
        mov     x1, #0xc                        // #12
        mul     x8, x20, x1
        umulh   x9, x20, x1
        adds    x3, x3, x8
        mul     x8, x25, x1
        umulh   x10, x25, x1
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
        adds    x20, x3, x8
        and     x9, x8, #0xffffffff
        adcs    x21, x4, x9
        adcs    x22, x5, xzr
        neg     x10, x9
        adc     x23, x6, x10
        stp     x20, x21, [sp, #160]
        stp     x22, x23, [sp, #176]
        mov     x2, sp
        ldp     x4, x3, [x2]
        subs    x5, x13, x4
        sbcs    x6, x26, x3
        ldp     x4, x3, [x2, #16]
        sbcs    x7, x14, x4
        sbcs    x8, x15, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [sp, #64]
        stp     x7, x8, [sp, #80]
        mov     x0, sp
        ldr     q19, [sp, #32]
        ldp     x9, x13, [sp, #32]
        ldr     q23, [sp, #48]
        ldr     q0, [sp, #32]
        ldp     x1, x10, [sp, #48]
        uzp2    v29.4s, v19.4s, v19.4s
        xtn     v4.2s, v19.2d
        umulh   x8, x9, x13
        rev64   v20.4s, v23.4s
        umull   v16.2d, v19.2s, v19.2s
        umull   v1.2d, v29.2s, v4.2s
        mul     v20.4s, v20.4s, v0.4s
        subs    x14, x9, x13
        umulh   x15, x9, x1
        mov     x16, v16.d[1]
        umull2  v4.2d, v19.4s, v19.4s
        mov     x4, v16.d[0]
        uzp1    v17.4s, v23.4s, v0.4s
        uaddlp  v19.2d, v20.4s
        lsr     x7, x8, #63
        mul     x11, x9, x13
        mov     x12, v1.d[0]
        csetm   x5, cc  // cc = lo, ul, last
        cneg    x6, x14, cc  // cc = lo, ul, last
        mov     x3, v4.d[1]
        mov     x14, v4.d[0]
        subs    x2, x10, x1
        mov     x9, v1.d[1]
        cneg    x17, x2, cc  // cc = lo, ul, last
        cinv    x2, x5, cc  // cc = lo, ul, last
        adds    x5, x4, x12, lsl #33
        extr    x4, x8, x11, #63
        lsr     x8, x12, #31
        uzp1    v20.4s, v0.4s, v0.4s
        shl     v19.2d, v19.2d, #32
        adc     x16, x16, x8
        adds    x8, x14, x9, lsl #33
        lsr     x14, x9, #31
        lsl     x9, x5, #32
        umlal   v19.2d, v20.2s, v17.2s
        adc     x14, x3, x14
        adds    x16, x16, x11, lsl #1
        lsr     x3, x5, #32
        umulh   x12, x6, x17
        adcs    x4, x8, x4
        adc     x11, x14, x7
        subs    x8, x5, x9
        sbc     x5, x5, x3
        adds    x16, x16, x9
        mov     x14, v19.d[0]
        mul     x17, x6, x17
        adcs    x3, x4, x3
        lsl     x7, x16, #32
        umulh   x13, x13, x10
        adcs    x11, x11, x8
        lsr     x8, x16, #32
        adc     x5, x5, xzr
        subs    x9, x16, x7
        sbc     x16, x16, x8
        adds    x7, x3, x7
        mov     x3, v19.d[1]
        adcs    x6, x11, x8
        umulh   x11, x1, x10
        adcs    x5, x5, x9
        eor     x8, x12, x2
        adc     x9, x16, xzr
        adds    x16, x14, x15
        adc     x15, x15, xzr
        adds    x12, x16, x3
        eor     x16, x17, x2
        mul     x4, x1, x10
        adcs    x15, x15, x13
        adc     x17, x13, xzr
        adds    x15, x15, x3
        adc     x3, x17, xzr
        cmn     x2, #0x1
        mul     x17, x10, x10
        adcs    x12, x12, x16
        adcs    x16, x15, x8
        umulh   x10, x10, x10
        adc     x2, x3, x2
        adds    x14, x14, x14
        adcs    x12, x12, x12
        adcs    x16, x16, x16
        adcs    x2, x2, x2
        adc     x15, xzr, xzr
        adds    x14, x14, x7
        mul     x3, x1, x1
        adcs    x12, x12, x6
        lsr     x7, x14, #32
        adcs    x16, x16, x5
        lsl     x5, x14, #32
        umulh   x13, x1, x1
        adcs    x2, x2, x9
        mov     x6, #0xffffffff                 // #4294967295
        adc     x15, x15, xzr
        adds    x8, x4, x4
        adcs    x1, x11, x11
        mov     x11, #0xffffffff00000001        // #-4294967295
        adc     x4, xzr, xzr
        subs    x9, x14, x5
        sbc     x14, x14, x7
        adds    x12, x12, x5
        adcs    x16, x16, x7
        lsl     x5, x12, #32
        lsr     x7, x12, #32
        adcs    x2, x2, x9
        adcs    x14, x15, x14
        adc     x15, xzr, xzr
        subs    x9, x12, x5
        sbc     x12, x12, x7
        adds    x16, x16, x5
        adcs    x2, x2, x7
        adcs    x14, x14, x9
        adcs    x12, x15, x12
        adc     x15, xzr, xzr
        adds    x16, x16, x3
        adcs    x2, x2, x13
        adcs    x14, x14, x17
        adcs    x12, x12, x10
        adc     x15, x15, xzr
        adds    x2, x2, x8
        adcs    x14, x14, x1
        adcs    x12, x12, x4
        adcs    x15, x15, xzr
        adds    x3, x16, #0x1
        sbcs    x5, x2, x6
        sbcs    x8, x14, xzr
        sbcs    x11, x12, x11
        sbcs    xzr, x15, xzr
        csel    x24, x3, x16, cs  // cs = hs, nlast
        csel    x25, x8, x14, cs  // cs = hs, nlast
        csel    x26, x11, x12, cs  // cs = hs, nlast
        csel    x27, x5, x2, cs  // cs = hs, nlast
        stp     x25, x26, [x0, #16]
        stp     x24, x27, [x0]
        ldr     q20, [sp, #96]
        ldr     q0, [sp, #160]
        ldp     x6, x10, [sp, #96]
        rev64   v16.4s, v20.4s
        subs    x4, x20, x21
        csetm   x3, cc  // cc = lo, ul, last
        cneg    x13, x4, cc  // cc = lo, ul, last
        mul     v16.4s, v16.4s, v0.4s
        umulh   x12, x21, x10
        uzp1    v28.4s, v20.4s, v0.4s
        subs    x14, x22, x20
        ldr     q20, [sp, #112]
        sbcs    x5, x23, x21
        ngc     x17, xzr
        subs    x8, x22, x23
        uaddlp  v27.2d, v16.4s
        umulh   x4, x20, x6
        uzp1    v21.4s, v0.4s, v0.4s
        cneg    x11, x8, cc  // cc = lo, ul, last
        shl     v17.2d, v27.2d, #32
        csetm   x15, cc  // cc = lo, ul, last
        subs    x9, x10, x6
        eor     x7, x14, x17
        umlal   v17.2d, v21.2s, v28.2s
        cneg    x8, x9, cc  // cc = lo, ul, last
        cinv    x9, x3, cc  // cc = lo, ul, last
        cmn     x17, #0x1
        ldr     q28, [sp, #176]
        adcs    x14, x7, xzr
        mul     x7, x13, x8
        eor     x1, x5, x17
        adcs    x5, x1, xzr
        xtn     v1.2s, v20.2d
        mov     x1, v17.d[0]
        mov     x3, v17.d[1]
        uzp2    v16.4s, v20.4s, v20.4s
        umulh   x16, x13, x8
        eor     x13, x7, x9
        adds    x8, x1, x3
        adcs    x7, x4, x12
        xtn     v0.2s, v28.2d
        adcs    x12, x12, xzr
        adds    x8, x4, x8
        adcs    x3, x3, x7
        ldp     x7, x2, [sp, #112]
        adcs    x12, x12, xzr
        cmn     x9, #0x1
        adcs    x8, x8, x13
        eor     x13, x16, x9
        adcs    x16, x3, x13
        lsl     x3, x1, #32
        adc     x13, x12, x9
        subs    x12, x6, x7
        sbcs    x9, x10, x2
        lsr     x10, x1, #32
        ngc     x4, xzr
        subs    x6, x2, x7
        cinv    x2, x15, cc  // cc = lo, ul, last
        cneg    x6, x6, cc  // cc = lo, ul, last
        subs    x7, x1, x3
        eor     x9, x9, x4
        sbc     x1, x1, x10
        adds    x15, x8, x3
        adcs    x3, x16, x10
        mul     x16, x11, x6
        adcs    x8, x13, x7
        eor     x13, x12, x4
        adc     x10, x1, xzr
        cmn     x4, #0x1
        umulh   x6, x11, x6
        adcs    x11, x13, xzr
        adcs    x1, x9, xzr
        lsl     x13, x15, #32
        subs    x12, x15, x13
        lsr     x7, x15, #32
        sbc     x15, x15, x7
        adds    x20, x3, x13
        adcs    x21, x8, x7
        umulh   x8, x14, x11
        umull   v21.2d, v0.2s, v1.2s
        adcs    x22, x10, x12
        umull   v3.2d, v0.2s, v16.2s
        adc     x23, x15, xzr
        rev64   v24.4s, v20.4s
        movi    v2.2d, #0xffffffff
        mul     x10, x14, x11
        mul     v4.4s, v24.4s, v28.4s
        subs    x13, x14, x5
        uzp2    v19.4s, v28.4s, v28.4s
        csetm   x15, cc  // cc = lo, ul, last
        usra    v3.2d, v21.2d, #32
        mul     x7, x5, x1
        umull   v21.2d, v19.2s, v16.2s
        cneg    x13, x13, cc  // cc = lo, ul, last
        uaddlp  v5.2d, v4.4s
        subs    x11, x1, x11
        and     v16.16b, v3.16b, v2.16b
        umulh   x5, x5, x1
        shl     v24.2d, v5.2d, #32
        cneg    x11, x11, cc  // cc = lo, ul, last
        umlal   v16.2d, v19.2s, v1.2s
        cinv    x12, x15, cc  // cc = lo, ul, last
        umlal   v24.2d, v0.2s, v1.2s
        adds    x15, x10, x7
        mul     x14, x13, x11
        eor     x1, x6, x2
        adcs    x6, x8, x5
        usra    v21.2d, v3.2d, #32
        adcs    x9, x5, xzr
        umulh   x11, x13, x11
        adds    x15, x8, x15
        adcs    x7, x7, x6
        eor     x8, x14, x12
        usra    v21.2d, v16.2d, #32
        adcs    x13, x9, xzr
        cmn     x12, #0x1
        mov     x9, v24.d[1]
        adcs    x14, x15, x8
        eor     x6, x11, x12
        adcs    x6, x7, x6
        mov     x5, v24.d[0]
        mov     x11, v21.d[1]
        mov     x7, v21.d[0]
        adc     x3, x13, x12
        adds    x12, x5, x9
        adcs    x13, x7, x11
        adcs    x11, x11, xzr
        adds    x12, x7, x12
        eor     x16, x16, x2
        adcs    x7, x9, x13
        adcs    x11, x11, xzr
        cmn     x2, #0x1
        adcs    x16, x12, x16
        adcs    x1, x7, x1
        adc     x2, x11, x2
        adds    x7, x5, x20
        adcs    x15, x16, x21
        eor     x5, x17, x4
        adcs    x9, x1, x22
        eor     x1, x10, x5
        adcs    x16, x2, x23
        adc     x2, xzr, xzr
        cmn     x5, #0x1
        eor     x13, x14, x5
        adcs    x14, x1, x7
        eor     x1, x6, x5
        adcs    x6, x13, x15
        adcs    x10, x1, x9
        eor     x4, x3, x5
        mov     x1, #0xffffffff                 // #4294967295
        adcs    x8, x4, x16
        lsr     x13, x14, #32
        adcs    x17, x2, x5
        adcs    x11, x5, xzr
        adc     x4, x5, xzr
        adds    x12, x10, x7
        adcs    x7, x8, x15
        adcs    x5, x17, x9
        adcs    x9, x11, x16
        lsl     x11, x14, #32
        adc     x10, x4, x2
        subs    x17, x14, x11
        sbc     x4, x14, x13
        adds    x11, x6, x11
        adcs    x12, x12, x13
        lsl     x15, x11, #32
        adcs    x17, x7, x17
        lsr     x7, x11, #32
        adc     x13, x4, xzr
        subs    x4, x11, x15
        sbc     x11, x11, x7
        adds    x8, x12, x15
        adcs    x15, x17, x7
        adcs    x4, x13, x4
        adc     x11, x11, xzr
        adds    x7, x5, x4
        adcs    x17, x9, x11
        adc     x13, x10, xzr
        add     x12, x13, #0x1
        neg     x11, x12
        lsl     x4, x12, #32
        adds    x17, x17, x4
        sub     x4, x4, #0x1
        adc     x13, x13, xzr
        subs    x11, x8, x11
        sbcs    x4, x15, x4
        sbcs    x7, x7, xzr
        sbcs    x17, x17, x12
        sbcs    x13, x13, x12
        mov     x12, #0xffffffff00000001        // #-4294967295
        adds    x14, x11, x13
        and     x1, x1, x13
        adcs    x15, x4, x1
        and     x1, x12, x13
        stp     x14, x15, [sp, #96]
        adcs    x13, x7, xzr
        adc     x20, x17, x1
        stp     x13, x20, [sp, #112]
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [sp, #32]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [sp, #48]
        sbcs    x7, x7, x4
        sbcs    x8, x8, x3
        csetm   x3, cc  // cc = lo, ul, last
        adds    x5, x5, x3
        and     x4, x3, #0xffffffff
        adcs    x6, x6, x4
        adcs    x7, x7, xzr
        and     x4, x3, #0xffffffff00000001
        adc     x8, x8, x4
        stp     x5, x6, [x19, #64]
        stp     x7, x8, [x19, #80]
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
        csetm   x5, cc  // cc = lo, ul, last
        adds    x0, x0, x5
        and     x6, x5, #0xffffffff
        adcs    x1, x1, x6
        adcs    x2, x2, xzr
        neg     x7, x6
        adc     x3, x3, x7
        stp     x0, x1, [x19]
        stp     x2, x3, [x19, #16]
        mov     x2, #0xffffffffffffffff         // #-1
        subs    x9, x2, x24
        mov     x2, #0xffffffff                 // #4294967295
        sbcs    x10, x2, x27
        ngcs    x11, x25
        mov     x2, #0xffffffff00000001         // #-4294967295
        sbc     x12, x2, x26
        lsl     x3, x9, #3
        extr    x4, x10, x9, #61
        extr    x5, x11, x10, #61
        extr    x6, x12, x11, #61
        lsr     x7, x12, #61
        mov     x1, #0x3                        // #3
        mul     x8, x14, x1
        umulh   x9, x14, x1
        adds    x3, x3, x8
        mul     x8, x15, x1
        umulh   x10, x15, x1
        adcs    x4, x4, x8
        mul     x8, x13, x1
        umulh   x11, x13, x1
        adcs    x5, x5, x8
        mul     x8, x20, x1
        umulh   x12, x20, x1
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
        stp     x3, x4, [x19, #32]
        stp     x5, x6, [x19, #48]
        ldp     x27, xzr, [sp, #256]
        ldp     x25, x26, [sp, #240]
        ldp     x23, x24, [sp, #224]
        ldp     x21, x22, [sp, #208]
        ldp     x19, x20, [sp, #192]
        CFI_INC_SP(272)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_p256_montjdouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_local_p256_montjmixadd)

Lp256_scalarmul_local_p256_montjmixadd:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_DEC_SP(192)
        mov     x17, x0
        mov     x19, x1
        mov     x20, x2
        ldp     x2, x3, [x19, #64]
        ldp     x4, x5, [x19, #80]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x0, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x0, x0, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x0, x0, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        subs    x13, x15, x12
        lsr     x11, x15, #32
        sbc     x15, x15, x11
        adds    x16, x16, x12
        adcs    x0, x0, x11
        adcs    x1, x1, x13
        adc     x15, x15, xzr
        lsl     x12, x16, #32
        subs    x13, x16, x12
        lsr     x11, x16, #32
        sbc     x16, x16, x11
        adds    x0, x0, x12
        adcs    x1, x1, x11
        adcs    x15, x15, x13
        adc     x16, x16, xzr
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, cc
        csetm   x13, cc
        subs    x12, x5, x4
        cneg    x12, x12, cc
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, cc
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x0
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        subs    x13, x6, x12
        lsr     x11, x6, #32
        sbc     x6, x6, x11
        adds    x7, x7, x12
        adcs    x8, x8, x11
        adcs    x9, x9, x13
        adcs    x10, x10, x6
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        subs    x13, x7, x12
        lsr     x11, x7, #32
        sbc     x7, x7, x11
        adds    x8, x8, x12
        adcs    x9, x9, x11
        adcs    x10, x10, x13
        adcs    x6, x6, x7
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #0xffffffff
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #0xffffffff00000001
        sbcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, cs
        csel    x9, x11, x9, cs
        csel    x10, x12, x10, cs
        csel    x6, x13, x6, cs
        stp     x8, x9, [sp]
        stp     x10, x6, [sp, #16]
        ldp     x3, x4, [x19, #64]
        ldp     x5, x6, [x19, #80]
        ldp     x7, x8, [x20, #32]
        ldp     x9, x10, [x20, #48]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #32]
        stp     x11, x12, [sp, #48]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #32]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #48]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #32]
        stp     x3, x4, [sp, #48]
        ldp     x3, x4, [sp]
        ldp     x5, x6, [sp, #16]
        ldp     x7, x8, [x20]
        ldp     x9, x10, [x20, #16]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #64]
        stp     x11, x12, [sp, #80]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #64]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #80]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #64]
        stp     x3, x4, [sp, #80]
        ldp     x3, x4, [sp]
        ldp     x5, x6, [sp, #16]
        ldp     x7, x8, [sp, #32]
        ldp     x9, x10, [sp, #48]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #32]
        stp     x11, x12, [sp, #48]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #32]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #48]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #32]
        stp     x3, x4, [sp, #48]
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [x19]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [x19, #16]
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
        ldp     x4, x3, [x19, #32]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #48]
        ldp     x4, x3, [x19, #48]
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
        ldp     x4, x5, [sp, #176]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x0, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x0, x0, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x0, x0, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        subs    x13, x15, x12
        lsr     x11, x15, #32
        sbc     x15, x15, x11
        adds    x16, x16, x12
        adcs    x0, x0, x11
        adcs    x1, x1, x13
        adc     x15, x15, xzr
        lsl     x12, x16, #32
        subs    x13, x16, x12
        lsr     x11, x16, #32
        sbc     x16, x16, x11
        adds    x0, x0, x12
        adcs    x1, x1, x11
        adcs    x15, x15, x13
        adc     x16, x16, xzr
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, cc
        csetm   x13, cc
        subs    x12, x5, x4
        cneg    x12, x12, cc
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, cc
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x0
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        subs    x13, x6, x12
        lsr     x11, x6, #32
        sbc     x6, x6, x11
        adds    x7, x7, x12
        adcs    x8, x8, x11
        adcs    x9, x9, x13
        adcs    x10, x10, x6
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        subs    x13, x7, x12
        lsr     x11, x7, #32
        sbc     x7, x7, x11
        adds    x8, x8, x12
        adcs    x9, x9, x11
        adcs    x10, x10, x13
        adcs    x6, x6, x7
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #0xffffffff
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #0xffffffff00000001
        sbcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, cs
        csel    x9, x11, x9, cs
        csel    x10, x12, x10, cs
        csel    x6, x13, x6, cs
        stp     x8, x9, [sp, #96]
        stp     x10, x6, [sp, #112]
        ldp     x2, x3, [sp, #32]
        ldp     x4, x5, [sp, #48]
        umull   x15, w2, w2
        lsr     x11, x2, #32
        umull   x16, w11, w11
        umull   x11, w2, w11
        adds    x15, x15, x11, lsl #33
        lsr     x11, x11, #31
        adc     x16, x16, x11
        umull   x0, w3, w3
        lsr     x11, x3, #32
        umull   x1, w11, w11
        umull   x11, w3, w11
        mul     x12, x2, x3
        umulh   x13, x2, x3
        adds    x0, x0, x11, lsl #33
        lsr     x11, x11, #31
        adc     x1, x1, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adc     x1, x1, xzr
        adds    x16, x16, x12
        adcs    x0, x0, x13
        adc     x1, x1, xzr
        lsl     x12, x15, #32
        subs    x13, x15, x12
        lsr     x11, x15, #32
        sbc     x15, x15, x11
        adds    x16, x16, x12
        adcs    x0, x0, x11
        adcs    x1, x1, x13
        adc     x15, x15, xzr
        lsl     x12, x16, #32
        subs    x13, x16, x12
        lsr     x11, x16, #32
        sbc     x16, x16, x11
        adds    x0, x0, x12
        adcs    x1, x1, x11
        adcs    x15, x15, x13
        adc     x16, x16, xzr
        mul     x6, x2, x4
        mul     x14, x3, x5
        umulh   x8, x2, x4
        subs    x10, x2, x3
        cneg    x10, x10, cc
        csetm   x13, cc
        subs    x12, x5, x4
        cneg    x12, x12, cc
        mul     x11, x10, x12
        umulh   x12, x10, x12
        cinv    x13, x13, cc
        eor     x11, x11, x13
        eor     x12, x12, x13
        adds    x7, x6, x8
        adc     x8, x8, xzr
        umulh   x9, x3, x5
        adds    x7, x7, x14
        adcs    x8, x8, x9
        adc     x9, x9, xzr
        adds    x8, x8, x14
        adc     x9, x9, xzr
        cmn     x13, #0x1
        adcs    x7, x7, x11
        adcs    x8, x8, x12
        adc     x9, x9, x13
        adds    x6, x6, x6
        adcs    x7, x7, x7
        adcs    x8, x8, x8
        adcs    x9, x9, x9
        adc     x10, xzr, xzr
        adds    x6, x6, x0
        adcs    x7, x7, x1
        adcs    x8, x8, x15
        adcs    x9, x9, x16
        adc     x10, x10, xzr
        lsl     x12, x6, #32
        subs    x13, x6, x12
        lsr     x11, x6, #32
        sbc     x6, x6, x11
        adds    x7, x7, x12
        adcs    x8, x8, x11
        adcs    x9, x9, x13
        adcs    x10, x10, x6
        adc     x6, xzr, xzr
        lsl     x12, x7, #32
        subs    x13, x7, x12
        lsr     x11, x7, #32
        sbc     x7, x7, x11
        adds    x8, x8, x12
        adcs    x9, x9, x11
        adcs    x10, x10, x13
        adcs    x6, x6, x7
        adc     x7, xzr, xzr
        mul     x11, x4, x4
        adds    x8, x8, x11
        mul     x12, x5, x5
        umulh   x11, x4, x4
        adcs    x9, x9, x11
        adcs    x10, x10, x12
        umulh   x12, x5, x5
        adcs    x6, x6, x12
        adc     x7, x7, xzr
        mul     x11, x4, x5
        umulh   x12, x4, x5
        adds    x11, x11, x11
        adcs    x12, x12, x12
        adc     x13, xzr, xzr
        adds    x9, x9, x11
        adcs    x10, x10, x12
        adcs    x6, x6, x13
        adcs    x7, x7, xzr
        mov     x11, #0xffffffff
        adds    x5, x8, #0x1
        sbcs    x11, x9, x11
        mov     x13, #0xffffffff00000001
        sbcs    x12, x10, xzr
        sbcs    x13, x6, x13
        sbcs    xzr, x7, xzr
        csel    x8, x5, x8, cs
        csel    x9, x11, x9, cs
        csel    x10, x12, x10, cs
        csel    x6, x13, x6, cs
        stp     x8, x9, [sp]
        stp     x10, x6, [sp, #16]
        ldp     x3, x4, [sp, #96]
        ldp     x5, x6, [sp, #112]
        ldp     x7, x8, [x19]
        ldp     x9, x10, [x19, #16]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #128]
        stp     x11, x12, [sp, #144]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #128]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #144]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #128]
        stp     x3, x4, [sp, #144]
        ldp     x3, x4, [sp, #96]
        ldp     x5, x6, [sp, #112]
        ldp     x7, x8, [sp, #64]
        ldp     x9, x10, [sp, #80]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #64]
        stp     x11, x12, [sp, #80]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #64]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #80]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #64]
        stp     x3, x4, [sp, #80]
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
        ldp     x5, x6, [sp, #176]
        ldp     x7, x8, [x19, #64]
        ldp     x9, x10, [x19, #80]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #160]
        stp     x11, x12, [sp, #176]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #160]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #176]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #160]
        stp     x3, x4, [sp, #176]
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
        ldp     x5, x6, [sp, #112]
        ldp     x7, x8, [x19, #32]
        ldp     x9, x10, [x19, #48]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #96]
        stp     x11, x12, [sp, #112]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #96]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #112]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #96]
        stp     x3, x4, [sp, #112]
        ldp     x3, x4, [sp, #32]
        ldp     x5, x6, [sp, #48]
        ldp     x7, x8, [sp, #128]
        ldp     x9, x10, [sp, #144]
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x3, x4
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        stp     x13, x14, [sp, #128]
        stp     x11, x12, [sp, #144]
        mul     x11, x5, x9
        mul     x13, x6, x10
        umulh   x12, x5, x9
        adds    x16, x11, x13
        umulh   x14, x6, x10
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x15, x5, x6
        cneg    x15, x15, cc
        csetm   x1, cc
        subs    x0, x10, x9
        cneg    x0, x0, cc
        mul     x16, x15, x0
        umulh   x0, x15, x0
        cinv    x1, x1, cc
        eor     x16, x16, x1
        eor     x0, x0, x1
        cmn     x1, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x1
        subs    x3, x5, x3
        sbcs    x4, x6, x4
        ngc     x5, xzr
        cmn     x5, #0x1
        eor     x3, x3, x5
        adcs    x3, x3, xzr
        eor     x4, x4, x5
        adcs    x4, x4, xzr
        subs    x7, x7, x9
        sbcs    x8, x8, x10
        ngc     x9, xzr
        cmn     x9, #0x1
        eor     x7, x7, x9
        adcs    x7, x7, xzr
        eor     x8, x8, x9
        adcs    x8, x8, xzr
        eor     x10, x5, x9
        ldp     x15, x1, [sp, #128]
        adds    x15, x11, x15
        adcs    x1, x12, x1
        ldp     x5, x9, [sp, #144]
        adcs    x5, x13, x5
        adcs    x9, x14, x9
        adc     x2, xzr, xzr
        mul     x11, x3, x7
        mul     x13, x4, x8
        umulh   x12, x3, x7
        adds    x16, x11, x13
        umulh   x14, x4, x8
        adcs    x0, x12, x14
        adcs    x14, x14, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x0
        adcs    x14, x14, xzr
        subs    x3, x3, x4
        cneg    x3, x3, cc
        csetm   x4, cc
        subs    x0, x8, x7
        cneg    x0, x0, cc
        mul     x16, x3, x0
        umulh   x0, x3, x0
        cinv    x4, x4, cc
        eor     x16, x16, x4
        eor     x0, x0, x4
        cmn     x4, #0x1
        adcs    x12, x12, x16
        adcs    x13, x13, x0
        adc     x14, x14, x4
        cmn     x10, #0x1
        eor     x11, x11, x10
        adcs    x11, x11, x15
        eor     x12, x12, x10
        adcs    x12, x12, x1
        eor     x13, x13, x10
        adcs    x13, x13, x5
        eor     x14, x14, x10
        adcs    x14, x14, x9
        adcs    x3, x2, x10
        adcs    x4, x10, xzr
        adc     x10, x10, xzr
        adds    x13, x13, x15
        adcs    x14, x14, x1
        adcs    x3, x3, x5
        adcs    x4, x4, x9
        adc     x10, x10, x2
        lsl     x0, x11, #32
        subs    x1, x11, x0
        lsr     x16, x11, #32
        sbc     x11, x11, x16
        adds    x12, x12, x0
        adcs    x13, x13, x16
        adcs    x14, x14, x1
        adc     x11, x11, xzr
        lsl     x0, x12, #32
        subs    x1, x12, x0
        lsr     x16, x12, #32
        sbc     x12, x12, x16
        adds    x13, x13, x0
        adcs    x14, x14, x16
        adcs    x11, x11, x1
        adc     x12, x12, xzr
        adds    x3, x3, x11
        adcs    x4, x4, x12
        adc     x10, x10, xzr
        add     x2, x10, #0x1
        lsl     x16, x2, #32
        adds    x4, x4, x16
        adc     x10, x10, xzr
        neg     x15, x2
        sub     x16, x16, #0x1
        subs    x13, x13, x15
        sbcs    x14, x14, x16
        sbcs    x3, x3, xzr
        sbcs    x4, x4, x2
        sbcs    x7, x10, x2
        adds    x13, x13, x7
        mov     x10, #0xffffffff
        and     x10, x10, x7
        adcs    x14, x14, x10
        adcs    x3, x3, xzr
        mov     x10, #0xffffffff00000001
        and     x10, x10, x7
        adc     x4, x4, x10
        stp     x13, x14, [sp, #128]
        stp     x3, x4, [sp, #144]
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
        ldp     x0, x1, [x19, #64]
        ldp     x2, x3, [x19, #80]
        orr     x4, x0, x1
        orr     x5, x2, x3
        orr     x4, x4, x5
        cmp     x4, xzr
        ldp     x0, x1, [sp]
        ldp     x12, x13, [x20]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x2, x3, [sp, #16]
        ldp     x12, x13, [x20, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x4, x5, [sp, #128]
        ldp     x12, x13, [x20, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x6, x7, [sp, #144]
        ldp     x12, x13, [x20, #48]
        csel    x6, x6, x12, ne
        csel    x7, x7, x13, ne
        ldp     x8, x9, [sp, #160]
        mov     x12, #0x1
        mov     x13, #0xffffffff00000000
        csel    x8, x8, x12, ne
        csel    x9, x9, x13, ne
        ldp     x10, x11, [sp, #176]
        mov     x12, #0xffffffffffffffff
        mov     x13, #0xfffffffe
        csel    x10, x10, x12, ne
        csel    x11, x11, x13, ne
        stp     x0, x1, [x17]
        stp     x2, x3, [x17, #16]
        stp     x4, x5, [x17, #32]
        stp     x6, x7, [x17, #48]
        stp     x8, x9, [x17, #64]
        stp     x10, x11, [x17, #80]
        CFI_INC_SP(192)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_local_p256_montjmixadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
