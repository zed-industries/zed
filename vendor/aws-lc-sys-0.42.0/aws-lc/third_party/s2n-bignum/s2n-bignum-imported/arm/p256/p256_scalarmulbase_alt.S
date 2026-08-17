// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Scalar multiplication for precomputed point on NIST curve P-256
// Input scalar[4], blocksize, table[]; output res[8]
//
// extern void p256_scalarmulbase_alt
//   (uint64_t res[static 8],
//    const uint64_t scalar[static 4],
//    uint64_t blocksize,
//    const uint64_t *table);
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-256, the input argument "table" is expected to be a table of
// multiples of the point P in Montgomery-affine form, with each block
// corresponding to "blocksize" bits of the scalar as follows, where
// B = 2^{blocksize-1} (e.g. B = 8 for blocksize = 4):
//
// For each i,j with blocksize * i <= 256 and 1 <= j <= B
// the multiple 2^{blocksize * i} * j * P is stored at
// tab[8 * (B * i + (j - 1))], considered as uint64_t pointers
// or tab + 64 * (B * i + (j - 1)) as byte pointers.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = blocksize, X3 = table
// ----------------------------------------------------------------------------


#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_scalarmulbase_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_scalarmulbase_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_scalarmulbase_alt)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Safe copies of inputs and additional variables, with some aliasing

#define res x19
#define blocksize x20
#define table x21
#define i x22
#define bf x23
#define cf x24
#define j x25

// Intermediate variables on the stack. The last z2, z3 values can
// safely be overlaid on "nacc", which is no longer needed at the end.
// Uppercase syntactic variants make x86_att version simpler to generate

#define rscalar sp, #(0*NUMSIZE)
#define acc sp, #(1*NUMSIZE)
#define nacc sp, #(4*NUMSIZE)
#define tabent sp, #(7*NUMSIZE)

#define z2 sp, #(4*NUMSIZE)
#define z3 sp, #(5*NUMSIZE)

#define NSPACE 9*NUMSIZE

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(p256_scalarmulbase_alt):
        CFI_START

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x30)
        CFI_DEC_SP(NSPACE)

// Preserve the input arguments except the scalar, since that gets absorbed
// immediately. The "table" value subsequently gets shifted up each iteration
// of the loop, while "res" and "blocksize" are static throughout.

        mov     res, x0
        mov     blocksize, x2
        mov     table, x3

// Load the digits of group order n_256 = [x15;x14;x13;x12]

        movbig(x12, #0xf3b9, #0xcac2, #0xfc63, #0x2551)
        movbig(x13, #0xbce6, #0xfaad, #0xa717, #0x9e84)
        mov     x14, #0xffffffffffffffff
        mov     x15, #0xffffffff00000000

// First, reduce the input scalar mod n_256, i.e. conditionally subtract n_256
// Store it to "rscalar" (reduced scalar)

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

        stp     x2, x3, [rscalar]
        stp     x4, x5, [rscalar+16]

// Initialize the accumulator to all zeros and the "carry flag" cf to 0

        stp     xzr, xzr, [acc]
        stp     xzr, xzr, [acc+16]
        stp     xzr, xzr, [acc+32]
        stp     xzr, xzr, [acc+48]
        stp     xzr, xzr, [acc+64]
        stp     xzr, xzr, [acc+80]
        mov     cf, xzr

// Main loop over {i >= 0 | blocksize * i <= 256}. Note the non-strict
// inequality, to allow top carry for any choices of blocksize.

        mov     i, xzr

Lp256_scalarmulbase_alt_loop:

// The next raw bitfield is bf = bitfield(blocksize * i,blocksize) + cf,
// adding in the deferred carry cf. We then shift the whole scalar right
// by blocksize so we can keep picking bitfield(0,blocksize).

        ldp     x0, x1, [rscalar]
        ldp     x2, x3, [rscalar+16]

        mov     x4, #1
        lsl     x4, x4, blocksize
        sub     x4, x4, #1
        and     x4, x4, x0
        add     bf, x4, cf

        neg     x8, blocksize

        lsl     x5, x1, x8

        lsr     x0, x0, blocksize
        orr     x0, x0,  x5

        lsl     x6, x2, x8
        lsr     x1, x1, blocksize
        orr     x1, x1, x6

        lsl     x7, x3, x8
        lsr     x2, x2, blocksize
        orr     x2, x2, x7

        lsr     x3, x3, blocksize

        stp     x0, x1, [rscalar]
        stp     x2, x3, [rscalar+16]

// Now if bf <= B we just select entry j, unnegated and set cf = 0.
// If bf > B we set j = 2 * B - bf and negate the j'th entry, setting cf = 1.
// In either case we ultimately add bf, in the latter case with deferred
// carry as 2 * B - (2 * B - bf) = bf.

        mov     x0, #1
        lsl     x1, x0, blocksize
        lsr     x0, x1, #1

        sub     x2, x1, bf

        cmp     x0, bf
        cset    cf, cc
        csel    j, x2, bf, cc

// Load table entry j - 1 for nonzero j in constant-time style.

        mov     x16, #1
        lsl     x16, x16, blocksize
        lsr     x16, x16, #1
        mov     x17, j

Lp256_scalarmulbase_alt_tabloop:
        ldp     x8, x9, [table]
        ldp     x10, x11, [table, #16]
        ldp     x12, x13, [table, #32]
        ldp     x14, x15, [table, #48]

        subs    x17, x17, #1
        csel    x0, x8, x0, eq
        csel    x1, x9, x1, eq
        csel    x2, x10, x2, eq
        csel    x3, x11, x3, eq
        csel    x4, x12, x4, eq
        csel    x5, x13, x5, eq
        csel    x6, x14, x6, eq
        csel    x7, x15, x7, eq

        add     table, table, #64

        sub     x16, x16, #1
        cbnz    x16, Lp256_scalarmulbase_alt_tabloop

// Before storing back, optionally negate the y coordinate of the table entry

        stp     x0, x1, [tabent]
        stp     x2, x3, [tabent+16]

        mov     x0, 0xffffffffffffffff
        subs    x0, x0, x4
        mov     x1, 0x00000000ffffffff
        sbcs    x1, x1, x5
        mov     x3, 0xffffffff00000001
        sbcs    x2, xzr, x6
        sbc     x3, x3, x7

        cmp     cf, xzr
        csel    x4, x0, x4, ne
        csel    x5, x1, x5, ne
        csel    x6, x2, x6, ne
        csel    x7, x3, x7, ne

        stp     x4, x5, [tabent+32]
        stp     x6, x7, [tabent+48]

// Add the adjusted table point to the accumulator

        add     x0, nacc
        add     x1, acc
        add     x2, tabent
        CFI_BL(Lp256_scalarmulbase_alt_local_p256_montjmixadd)

// However, only commit that update to the accumulator if j is nonzero,
// because the mixed addition function does not handle this case directly,
// and in any case we didn't choose the table entry appropriately.

        cmp     j, xzr
        ldp     x0, x1, [acc]
        ldp     x12, x13, [nacc]
        csel    x0, x12, x0, ne
        csel    x1, x13, x1, ne

        ldp     x2, x3, [acc+16]
        ldp     x12, x13, [nacc+16]
        csel    x2, x12, x2, ne
        csel    x3, x13, x3, ne

        ldp     x4, x5, [acc+32]
        ldp     x12, x13, [nacc+32]
        csel    x4, x12, x4, ne
        csel    x5, x13, x5, ne

        ldp     x6, x7, [acc+48]
        ldp     x12, x13, [nacc+48]
        csel    x6, x12, x6, ne
        csel    x7, x13, x7, ne

        ldp     x8, x9, [acc+64]
        ldp     x12, x13, [nacc+64]
        csel    x8, x12, x8, ne
        csel    x9, x13, x9, ne

        ldp     x10, x11, [acc+80]
        ldp     x12, x13, [nacc+80]
        csel    x10, x12, x10, ne
        csel    x11, x13, x11, ne

        stp     x0, x1, [acc]
        stp     x2, x3, [acc+16]
        stp     x4, x5, [acc+32]
        stp     x6, x7, [acc+48]
        stp     x8, x9, [acc+64]
        stp     x10, x11, [acc+80]

// Loop while blocksize * i <= 256

        add     i, i, #1
        mul     x0, blocksize, i
        cmp     x0, #257
        bcc     Lp256_scalarmulbase_alt_loop

// That's the end of the main loop, and we just need to translate
// back from the Jacobian representation to affine. First of all,
// let z2 = 1/z^2 and z3 = 1/z^3, both without Montgomery form

        add     x0, z2
        add     x1, acc+64
        CFI_BL(Lp256_scalarmulbase_alt_local_montsqr_p256)

        add     x0, z3
        add     x1, acc+64
        add     x2, z2
        CFI_BL(Lp256_scalarmulbase_alt_local_montmul_p256)

        add     x0, z2
        add     x1, z3
        CFI_BL(Lp256_scalarmulbase_alt_local_demont_p256)

        add     x0, z3
        add     x1, z2
        CFI_BL(Lp256_scalarmulbase_alt_local_inv_p256)

        add     x0, z2
        add     x1, acc+64
        add     x2, z3
        CFI_BL(Lp256_scalarmulbase_alt_local_montmul_p256)

// Convert back from Jacobian (X,Y,Z) |-> (X/Z^2, Y/Z^3)

        mov     x0, res
        add     x1, acc
        add     x2, z2
        CFI_BL(Lp256_scalarmulbase_alt_local_montmul_p256)

        add     x0, res, #32
        add     x1, acc+32
        add     x2, z3
        CFI_BL(Lp256_scalarmulbase_alt_local_montmul_p256)

// Restore stack and registers and return

        CFI_INC_SP(NSPACE)
        CFI_POP2(x25,x30)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_scalarmulbase_alt)

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmulbase_alt_local_demont_p256)

Lp256_scalarmulbase_alt_local_demont_p256:
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

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmulbase_alt_local_demont_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmulbase_alt_local_inv_p256)

Lp256_scalarmulbase_alt_local_inv_p256:
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
        b       Lp256_scalarmulbase_alt_inv_midloop
Lp256_scalarmulbase_alt_inv_loop:
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
Lp256_scalarmulbase_alt_inv_midloop:
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
        bne     Lp256_scalarmulbase_alt_inv_loop
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

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmulbase_alt_local_inv_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmulbase_alt_local_montmul_p256)

Lp256_scalarmulbase_alt_local_montmul_p256:
        CFI_START
        ldp     x3, x4, [x1]
        ldp     x7, x8, [x2]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x2, #16]
        mul     x11, x3, x9
        umulh   x15, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x16, x3, x10
        adcs    x15, x15, x11
        adc     x16, x16, xzr
        ldp     x5, x6, [x1, #16]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x15, x15, x11
        mul     x11, x4, x10
        adcs    x16, x16, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x15, x15, x11
        umulh   x11, x4, x9
        adcs    x16, x16, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x15, x15, x11
        mul     x11, x5, x9
        adcs    x16, x16, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x15, x15, x11
        umulh   x11, x5, x8
        adcs    x16, x16, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x15, x15, x11
        mul     x11, x6, x8
        adcs    x16, x16, x11
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
        adcs    x15, x15, x11
        adc     x12, x12, xzr
        umulh   x11, x6, x7
        adds    x16, x16, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        adds    x14, x14, x13, lsl #32
        lsr     x11, x13, #32
        adcs    x15, x15, x11
        mul     x11, x13, x10
        umulh   x13, x13, x10
        adcs    x12, x12, x11
        adc     x13, x13, xzr
        adds    x15, x15, x14, lsl #32
        lsr     x11, x14, #32
        adcs    x12, x12, x11
        mul     x11, x14, x10
        umulh   x14, x14, x10
        adcs    x13, x13, x11
        adc     x14, x14, xzr
        adds    x12, x12, x15, lsl #32
        lsr     x11, x15, #32
        adcs    x13, x13, x11
        mul     x11, x15, x10
        umulh   x15, x15, x10
        adcs    x14, x14, x11
        adc     x15, x15, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x3
        adcs    x14, x14, x4
        adcs    x15, x15, x5
        cset    x8, cs
        mov     x11, #0xffffffff
        adds    x16, x12, #0x1
        sbcs    x3, x13, x11
        sbcs    x4, x14, xzr
        sbcs    x5, x15, x10
        sbcs    xzr, x8, xzr
        csel    x12, x12, x16, cc
        csel    x13, x13, x3, cc
        csel    x14, x14, x4, cc
        csel    x15, x15, x5, cc
        stp     x12, x13, [x0]
        stp     x14, x15, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmulbase_alt_local_montmul_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmulbase_alt_local_montsqr_p256)

Lp256_scalarmulbase_alt_local_montsqr_p256:
        CFI_START
        ldp     x2, x3, [x1]
        mul     x9, x2, x3
        umulh   x10, x2, x3
        ldp     x4, x5, [x1, #16]
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
        lsr     x2, x8, #32
        adcs    x10, x10, x2
        mul     x2, x8, x5
        umulh   x8, x8, x5
        adcs    x11, x11, x2
        adc     x8, x8, xzr
        adds    x10, x10, x9, lsl #32
        lsr     x2, x9, #32
        adcs    x11, x11, x2
        mul     x2, x9, x5
        umulh   x9, x9, x5
        adcs    x8, x8, x2
        adc     x9, x9, xzr
        adds    x11, x11, x10, lsl #32
        lsr     x2, x10, #32
        adcs    x8, x8, x2
        mul     x2, x10, x5
        umulh   x10, x10, x5
        adcs    x9, x9, x2
        adc     x10, x10, xzr
        adds    x8, x8, x11, lsl #32
        lsr     x2, x11, #32
        adcs    x9, x9, x2
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
        stp     x8, x9, [x0]
        stp     x10, x11, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmulbase_alt_local_montsqr_p256)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmulbase_alt_local_p256_montjmixadd)

Lp256_scalarmulbase_alt_local_p256_montjmixadd:
        CFI_START
        CFI_DEC_SP(192)
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
        ldp     x5, x6, [sp, #64]
        ldp     x4, x3, [x16]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #80]
        ldp     x4, x3, [x16, #16]
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
        ldp     x4, x3, [x16, #32]
        subs    x5, x5, x4
        sbcs    x6, x6, x3
        ldp     x7, x8, [sp, #48]
        ldp     x4, x3, [x16, #48]
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
        orr     x4, x0, x1
        orr     x5, x2, x3
        orr     x4, x4, x5
        cmp     x4, xzr
        ldp     x0, x1, [sp]
        ldp     x12, x13, [x17]
        csel    x0, x0, x12, ne
        csel    x1, x1, x13, ne
        ldp     x2, x3, [sp, #16]
        ldp     x12, x13, [x17, #16]
        csel    x2, x2, x12, ne
        csel    x3, x3, x13, ne
        ldp     x4, x5, [sp, #128]
        ldp     x12, x13, [x17, #32]
        csel    x4, x4, x12, ne
        csel    x5, x5, x13, ne
        ldp     x6, x7, [sp, #144]
        ldp     x12, x13, [x17, #48]
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
        stp     x0, x1, [x15]
        stp     x2, x3, [x15, #16]
        stp     x4, x5, [x15, #32]
        stp     x6, x7, [x15, #48]
        stp     x8, x9, [x15, #64]
        stp     x10, x11, [x15, #80]
        CFI_INC_SP(192)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmulbase_alt_local_p256_montjmixadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
