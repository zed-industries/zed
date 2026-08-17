// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jmixadd(uint64_t p3[static 27], const uint64_t p1[static 27],
//                             const uint64_t p2[static 18]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity. It is assumed that
// all the coordinates of the input points p1 and p2 are fully reduced
// mod p_521, that the z coordinate of p1 is nonzero and that neither
// p1 =~= p2 or p1 =~= -p2, where "=~=" means "represents the same affine
// point as".
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jmixadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jmixadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jmixadd)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 72

// Stable homes for input arguments during main code sequence

#define input_z x26
#define input_x x27
#define input_y x28

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_2 input_y, #0
#define y_2 input_y, #NUMSIZE

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries, with some aliasing
// NSPACE is the total stack needed for these temporaries

#define zp2 sp, #(NUMSIZE*0)
#define ww sp, #(NUMSIZE*0)
#define resx sp, #(NUMSIZE*0)

#define yd sp, #(NUMSIZE*1)
#define y2a sp, #(NUMSIZE*1)

#define x2a sp, #(NUMSIZE*2)
#define zzx2 sp, #(NUMSIZE*2)

#define zz sp, #(NUMSIZE*3)
#define t1 sp, #(NUMSIZE*3)

#define t2 sp, #(NUMSIZE*4)
#define zzx1 sp, #(NUMSIZE*4)
#define resy sp, #(NUMSIZE*4)

#define xd sp, #(NUMSIZE*5)
#define resz sp, #(NUMSIZE*5)

#define tmp sp, #(NUMSIZE*6)

#define NSPACE 512

// For the three field operations, we use subroutines not inlining.
// Call local code very close to bignum_mul_p521 and bignum_sqr_p521
// and bignum_sub_p521

#define mul_p521(P0,P1,P2)                      \
        add     x0, P0 __LF                        \
        add     x1, P1 __LF                        \
        add     x2, P2 __LF                        \
        CFI_BL(Lp521_jmixadd_local_mul_p521)

#define sqr_p521(P0,P1)                         \
        add     x0, P0 __LF                        \
        add     x1, P1 __LF                        \
        CFI_BL(Lp521_jmixadd_local_sqr_p521)

#define sub_p521(P0,P1,P2)                      \
        add     x0, P0 __LF                        \
        add     x1, P1 __LF                        \
        add     x2, P2 __LF                        \
        CFI_BL(Lp521_jmixadd_local_sub_p521)

S2N_BN_SYMBOL(p521_jmixadd):
        CFI_START

// Save regs and make room on stack for temporary variables

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_PUSH2(x29,x30)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1
        mov     input_y, x2

// Main code, just a sequence of basic field operations

        sqr_p521(zp2,z_1)
        mul_p521(y2a,z_1,y_2)

        mul_p521(x2a,zp2,x_2)
        mul_p521(y2a,zp2,y2a)

        sub_p521(xd,x2a,x_1)
        sub_p521(yd,y2a,y_1)

        sqr_p521(zz,xd)
        sqr_p521(ww,yd)

        mul_p521(zzx1,zz,x_1)
        mul_p521(zzx2,zz,x2a)

        sub_p521(resx,ww,zzx1)
        sub_p521(t1,zzx2,zzx1)

        mul_p521(resz,xd,z_1)

        sub_p521(resx,resx,zzx2)

        sub_p521(t2,zzx1,resx)

        mul_p521(t1,t1,y_1)
        mul_p521(t2,yd,t2)

        sub_p521(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        ldp     x0, x1, [z_1]
        orr     x0, x0, x1
        ldp     x2, x3, [z_1+16]
        orr     x2, x2, x3
        ldp     x4, x5, [z_1+32]
        orr     x4, x4, x5
        ldp     x6, x7, [z_1+48]
        orr     x6, x6, x7
        ldr     x8, [z_1+64]
        orr     x0, x0, x2
        orr     x4, x4, x6
        orr     x0, x0, x4
        orr     x0, x0, x8
        cmp     x0, xzr

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with an extra z = 1
// coordinate, hence giving 0 + p2 = p2 for the final result.

        ldp     x0, x1, [resx]
        ldp     x20, x21, [x_2]
        csel    x0, x0, x20, ne
        csel    x1, x1, x21, ne
        ldp     x2, x3, [resx+16]
        ldp     x20, x21, [x_2+16]
        csel    x2, x2, x20, ne
        csel    x3, x3, x21, ne
        ldp     x4, x5, [resx+32]
        ldp     x20, x21, [x_2+32]
        csel    x4, x4, x20, ne
        csel    x5, x5, x21, ne
        ldp     x6, x7, [resx+48]
        ldp     x20, x21, [x_2+48]
        csel    x6, x6, x20, ne
        csel    x7, x7, x21, ne
        ldr     x8, [resx+64]
        ldr     x20, [x_2+64]
        csel    x8, x8, x20, ne

        ldp     x10, x11, [resy]
        ldp     x20, x21, [y_2]
        csel    x10, x10, x20, ne
        csel    x11, x11, x21, ne
        ldp     x12, x13, [resy+16]
        ldp     x20, x21, [y_2+16]
        csel    x12, x12, x20, ne
        csel    x13, x13, x21, ne
        ldp     x14, x15, [resy+32]
        ldp     x20, x21, [y_2+32]
        csel    x14, x14, x20, ne
        csel    x15, x15, x21, ne
        ldp     x16, x17, [resy+48]
        ldp     x20, x21, [y_2+48]
        csel    x16, x16, x20, ne
        csel    x17, x17, x21, ne
        ldr     x19, [resy+64]
        ldr     x20, [y_2+64]
        csel    x19, x19, x20, ne

        stp     x0, x1, [x_3]
        stp     x2, x3, [x_3+16]
        stp     x4, x5, [x_3+32]
        stp     x6, x7, [x_3+48]
        str     x8, [x_3+64]
        stp     x10, x11, [y_3]
        stp     x12, x13, [y_3+16]
        stp     x14, x15, [y_3+32]
        stp     x16, x17, [y_3+48]
        str     x19, [y_3+64]

        ldp     x0, x1, [resz]
        mov     x20, #1
        csel    x0, x0, x20, ne
        csel    x1, x1, xzr, ne
        ldp     x2, x3, [resz+16]
        csel    x2, x2, xzr, ne
        csel    x3, x3, xzr, ne
        ldp     x4, x5, [resz+32]
        csel    x4, x4, xzr, ne
        csel    x5, x5, xzr, ne
        ldp     x6, x7, [resz+48]
        csel    x6, x6, xzr, ne
        csel    x7, x7, xzr, ne
        ldr     x8, [resz+64]
        csel    x8, x8, xzr, ne

        stp     x0, x1, [z_3]
        stp     x2, x3, [z_3+16]
        stp     x4, x5, [z_3+32]
        stp     x6, x7, [z_3+48]
        str     x8, [z_3+64]

// Restore stack and registers

        CFI_INC_SP(NSPACE)

        CFI_POP2(x29,x30)
        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p521_jmixadd)

// Local versions of the three field operations, almost identical to
// bignum_mul_p521, bignum_sqr_p521 and bignum_sub_p521 except for
// avoiding all intial register save-restore, and in the case of
// local_mul_p521, using the tmp buffer as temporary storage and
// avoiding x26.

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jmixadd_local_mul_p521)

Lp521_jmixadd_local_mul_p521:
        CFI_START
        ldp     x3, x4, [x1]
        ldp     x5, x6, [x1, #16]
        ldp     x7, x8, [x2]
        ldp     x9, x10, [x2, #16]
        mul     x11, x3, x7
        mul     x15, x4, x8
        mul     x16, x5, x9
        mul     x17, x6, x10
        umulh   x19, x3, x7
        adds    x15, x15, x19
        umulh   x19, x4, x8
        adcs    x16, x16, x19
        umulh   x19, x5, x9
        adcs    x17, x17, x19
        umulh   x19, x6, x10
        adc     x19, x19, xzr
        adds    x12, x15, x11
        adcs    x15, x16, x15
        adcs    x16, x17, x16
        adcs    x17, x19, x17
        adc     x19, xzr, x19
        adds    x13, x15, x11
        adcs    x14, x16, x12
        adcs    x15, x17, x15
        adcs    x16, x19, x16
        adcs    x17, xzr, x17
        adc     x19, xzr, x19
        subs    x24, x5, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x9
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x16, x16, x23
        eor     x22, x22, x21
        adcs    x17, x17, x22
        adc     x19, x19, x21
        subs    x24, x3, x4
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x8, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x12, x12, x23
        eor     x22, x22, x21
        adcs    x13, x13, x22
        adcs    x14, x14, x21
        adcs    x15, x15, x21
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x4, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x8
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x15, x15, x23
        eor     x22, x22, x21
        adcs    x16, x16, x22
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x3, x5
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x9, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x13, x13, x23
        eor     x22, x22, x21
        adcs    x14, x14, x22
        adcs    x15, x15, x21
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x3, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x14, x14, x23
        eor     x22, x22, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x4, x5
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x9, x8
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x14, x14, x23
        eor     x22, x22, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        lsl     x21, x11, #9
        extr    x11, x12, x11, #55
        extr    x12, x13, x12, #55
        extr    x13, x14, x13, #55
        lsr     x14, x14, #55
        ldp     x3, x4, [x1, #32]
        ldp     x5, x6, [x1, #48]
        ldp     x7, x8, [x2, #32]
        ldp     x9, x10, [x2, #48]
        stp     x15, x16, [tmp]
        stp     x17, x19, [tmp+16]
        stp     x21, x11, [tmp+32]
        stp     x12, x13, [tmp+48]
        str     x14, [tmp+64]
        mul     x11, x3, x7
        mul     x15, x4, x8
        mul     x16, x5, x9
        mul     x17, x6, x10
        umulh   x19, x3, x7
        adds    x15, x15, x19
        umulh   x19, x4, x8
        adcs    x16, x16, x19
        umulh   x19, x5, x9
        adcs    x17, x17, x19
        umulh   x19, x6, x10
        adc     x19, x19, xzr
        adds    x12, x15, x11
        adcs    x15, x16, x15
        adcs    x16, x17, x16
        adcs    x17, x19, x17
        adc     x19, xzr, x19
        adds    x13, x15, x11
        adcs    x14, x16, x12
        adcs    x15, x17, x15
        adcs    x16, x19, x16
        adcs    x17, xzr, x17
        adc     x19, xzr, x19
        subs    x24, x5, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x9
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x16, x16, x23
        eor     x22, x22, x21
        adcs    x17, x17, x22
        adc     x19, x19, x21
        subs    x24, x3, x4
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x8, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x12, x12, x23
        eor     x22, x22, x21
        adcs    x13, x13, x22
        adcs    x14, x14, x21
        adcs    x15, x15, x21
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x4, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x8
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x15, x15, x23
        eor     x22, x22, x21
        adcs    x16, x16, x22
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x3, x5
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x9, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x13, x13, x23
        eor     x22, x22, x21
        adcs    x14, x14, x22
        adcs    x15, x15, x21
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x3, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x14, x14, x23
        eor     x22, x22, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x4, x5
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x9, x8
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x14, x14, x23
        eor     x22, x22, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        ldp     x23, x22, [tmp]
        adds    x11, x11, x23
        adcs    x12, x12, x22
        stp     x11, x12, [tmp]
        ldp     x23, x22, [tmp+16]
        adcs    x13, x13, x23
        adcs    x14, x14, x22
        stp     x13, x14, [tmp+16]
        ldp     x23, x22, [tmp+32]
        adcs    x15, x15, x23
        adcs    x16, x16, x22
        stp     x15, x16, [tmp+32]
        ldp     x23, x22, [tmp+48]
        adcs    x17, x17, x23
        adcs    x19, x19, x22
        stp     x17, x19, [tmp+48]
        ldr     x21, [tmp+64]
        adc     x21, x21, xzr
        str     x21, [tmp+64]
        ldp     x23, x22, [x1]
        subs    x3, x3, x23
        sbcs    x4, x4, x22
        ldp     x23, x22, [x1, #16]
        sbcs    x5, x5, x23
        sbcs    x6, x6, x22
        csetm   x24, lo
        ldp     x23, x22, [x2]
        subs    x7, x23, x7
        sbcs    x8, x22, x8
        ldp     x23, x22, [x2, #16]
        sbcs    x9, x23, x9
        sbcs    x10, x22, x10
        csetm   x25, lo
        eor     x3, x3, x24
        subs    x3, x3, x24
        eor     x4, x4, x24
        sbcs    x4, x4, x24
        eor     x5, x5, x24
        sbcs    x5, x5, x24
        eor     x6, x6, x24
        sbc     x6, x6, x24
        eor     x7, x7, x25
        subs    x7, x7, x25
        eor     x8, x8, x25
        sbcs    x8, x8, x25
        eor     x9, x9, x25
        sbcs    x9, x9, x25
        eor     x10, x10, x25
        sbc     x10, x10, x25
        eor     x25, x25, x24
        mul     x11, x3, x7
        mul     x15, x4, x8
        mul     x16, x5, x9
        mul     x17, x6, x10
        umulh   x19, x3, x7
        adds    x15, x15, x19
        umulh   x19, x4, x8
        adcs    x16, x16, x19
        umulh   x19, x5, x9
        adcs    x17, x17, x19
        umulh   x19, x6, x10
        adc     x19, x19, xzr
        adds    x12, x15, x11
        adcs    x15, x16, x15
        adcs    x16, x17, x16
        adcs    x17, x19, x17
        adc     x19, xzr, x19
        adds    x13, x15, x11
        adcs    x14, x16, x12
        adcs    x15, x17, x15
        adcs    x16, x19, x16
        adcs    x17, xzr, x17
        adc     x19, xzr, x19
        subs    x24, x5, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x9
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x16, x16, x23
        eor     x22, x22, x21
        adcs    x17, x17, x22
        adc     x19, x19, x21
        subs    x24, x3, x4
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x8, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x12, x12, x23
        eor     x22, x22, x21
        adcs    x13, x13, x22
        adcs    x14, x14, x21
        adcs    x15, x15, x21
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x4, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x8
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x15, x15, x23
        eor     x22, x22, x21
        adcs    x16, x16, x22
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x3, x5
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x9, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x13, x13, x23
        eor     x22, x22, x21
        adcs    x14, x14, x22
        adcs    x15, x15, x21
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x3, x6
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x10, x7
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x14, x14, x23
        eor     x22, x22, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        subs    x24, x4, x5
        cneg    x24, x24, lo
        csetm   x21, lo
        subs    x22, x9, x8
        cneg    x22, x22, lo
        mul     x23, x24, x22
        umulh   x22, x24, x22
        cinv    x21, x21, lo
        cmn     x21, #1
        eor     x23, x23, x21
        adcs    x14, x14, x23
        eor     x22, x22, x21
        adcs    x15, x15, x22
        adcs    x16, x16, x21
        adcs    x17, x17, x21
        adc     x19, x19, x21
        ldp     x3, x4, [tmp]
        ldp     x5, x6, [tmp+16]
        eor     x11, x11, x25
        adds    x11, x11, x3
        eor     x12, x12, x25
        adcs    x12, x12, x4
        eor     x13, x13, x25
        adcs    x13, x13, x5
        eor     x14, x14, x25
        adcs    x14, x14, x6
        eor     x15, x15, x25
        ldp     x7, x8, [tmp+32]
        ldp     x9, x10, [tmp+48]
        ldr     x20, [tmp+64]
        adcs    x15, x15, x7
        eor     x16, x16, x25
        adcs    x16, x16, x8
        eor     x17, x17, x25
        adcs    x17, x17, x9
        eor     x19, x19, x25
        adcs    x19, x19, x10
        adc     x21, x20, xzr
        adds    x15, x15, x3
        adcs    x16, x16, x4
        adcs    x17, x17, x5
        adcs    x19, x19, x6
        and     x25, x25, #0x1ff
        lsl     x24, x11, #9
        orr     x24, x24, x25
        adcs    x7, x7, x24
        extr    x24, x12, x11, #55
        adcs    x8, x8, x24
        extr    x24, x13, x12, #55
        adcs    x9, x9, x24
        extr    x24, x14, x13, #55
        adcs    x10, x10, x24
        lsr     x24, x14, #55
        adc     x20, x24, x20
        ldr     x6, [x2, #64]
        ldp     x3, x4, [x1]
        and     x23, x3, #0xfffffffffffff
        mul     x23, x6, x23
        ldr     x14, [x1, #64]
        ldp     x11, x12, [x2]
        and     x24, x11, #0xfffffffffffff
        mul     x24, x14, x24
        add     x23, x23, x24
        extr    x24, x4, x3, #52
        and     x24, x24, #0xfffffffffffff
        mul     x22, x6, x24
        extr    x24, x12, x11, #52
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x22, x22, x24
        lsr     x24, x23, #52
        add     x22, x22, x24
        lsl     x23, x23, #12
        extr    x24, x22, x23, #12
        adds    x15, x15, x24
        ldp     x5, x3, [x1, #16]
        ldp     x13, x11, [x2, #16]
        extr    x24, x5, x4, #40
        and     x24, x24, #0xfffffffffffff
        mul     x23, x6, x24
        extr    x24, x13, x12, #40
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x23, x23, x24
        lsr     x24, x22, #52
        add     x23, x23, x24
        lsl     x22, x22, #12
        extr    x24, x23, x22, #24
        adcs    x16, x16, x24
        extr    x24, x3, x5, #28
        and     x24, x24, #0xfffffffffffff
        mul     x22, x6, x24
        extr    x24, x11, x13, #28
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x22, x22, x24
        lsr     x24, x23, #52
        add     x22, x22, x24
        lsl     x23, x23, #12
        extr    x24, x22, x23, #36
        adcs    x17, x17, x24
        and     x25, x16, x17
        ldp     x4, x5, [x1, #32]
        ldp     x12, x13, [x2, #32]
        extr    x24, x4, x3, #16
        and     x24, x24, #0xfffffffffffff
        mul     x23, x6, x24
        extr    x24, x12, x11, #16
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x23, x23, x24
        lsl     x21, x21, #48
        add     x23, x23, x21
        lsr     x24, x22, #52
        add     x23, x23, x24
        lsl     x22, x22, #12
        extr    x24, x23, x22, #48
        adcs    x19, x19, x24
        and     x25, x25, x19
        lsr     x24, x4, #4
        and     x24, x24, #0xfffffffffffff
        mul     x22, x6, x24
        lsr     x24, x12, #4
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x22, x22, x24
        lsr     x24, x23, #52
        add     x22, x22, x24
        lsl     x23, x23, #12
        extr    x21, x22, x23, #60
        extr    x24, x5, x4, #56
        and     x24, x24, #0xfffffffffffff
        mul     x23, x6, x24
        extr    x24, x13, x12, #56
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x23, x23, x24
        lsr     x24, x22, #52
        add     x23, x23, x24
        lsl     x21, x21, #8
        extr    x24, x23, x21, #8
        adcs    x7, x7, x24
        and     x25, x25, x7
        ldp     x3, x4, [x1, #48]
        ldp     x11, x12, [x2, #48]
        extr    x24, x3, x5, #44
        and     x24, x24, #0xfffffffffffff
        mul     x22, x6, x24
        extr    x24, x11, x13, #44
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x22, x22, x24
        lsr     x24, x23, #52
        add     x22, x22, x24
        lsl     x23, x23, #12
        extr    x24, x22, x23, #20
        adcs    x8, x8, x24
        and     x25, x25, x8
        extr    x24, x4, x3, #32
        and     x24, x24, #0xfffffffffffff
        mul     x23, x6, x24
        extr    x24, x12, x11, #32
        and     x24, x24, #0xfffffffffffff
        mul     x24, x14, x24
        add     x23, x23, x24
        lsr     x24, x22, #52
        add     x23, x23, x24
        lsl     x22, x22, #12
        extr    x24, x23, x22, #32
        adcs    x9, x9, x24
        and     x25, x25, x9
        lsr     x24, x4, #20
        mul     x22, x6, x24
        lsr     x24, x12, #20
        mul     x24, x14, x24
        add     x22, x22, x24
        lsr     x24, x23, #52
        add     x22, x22, x24
        lsl     x23, x23, #12
        extr    x24, x22, x23, #44
        adcs    x10, x10, x24
        and     x25, x25, x10
        mul     x24, x6, x14
        lsr     x22, x22, #44
        add     x24, x24, x22
        adc     x20, x20, x24
        lsr     x22, x20, #9
        orr     x20, x20, #0xfffffffffffffe00
        cmp     xzr, xzr
        adcs    xzr, x15, x22
        adcs    xzr, x25, xzr
        adcs    xzr, x20, xzr
        adcs    x15, x15, x22
        adcs    x16, x16, xzr
        adcs    x17, x17, xzr
        adcs    x19, x19, xzr
        adcs    x7, x7, xzr
        adcs    x8, x8, xzr
        adcs    x9, x9, xzr
        adcs    x10, x10, xzr
        adc     x20, x20, xzr
        and     x22, x15, #0x1ff
        extr    x15, x16, x15, #9
        extr    x16, x17, x16, #9
        stp     x15, x16, [x0]
        extr    x17, x19, x17, #9
        extr    x19, x7, x19, #9
        stp     x17, x19, [x0, #16]
        extr    x7, x8, x7, #9
        extr    x8, x9, x8, #9
        stp     x7, x8, [x0, #32]
        extr    x9, x10, x9, #9
        extr    x10, x20, x10, #9
        stp     x9, x10, [x0, #48]
        str     x22, [x0, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jmixadd_local_mul_p521)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jmixadd_local_sqr_p521)

Lp521_jmixadd_local_sqr_p521:
        CFI_START
        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]
        ldp     x6, x7, [x1, #32]
        ldp     x8, x9, [x1, #48]
        mul     x12, x6, x8
        mul     x17, x7, x9
        umulh   x22, x6, x8
        subs    x23, x6, x7
        cneg    x23, x23, cc
        csetm   x11, cc
        subs    x10, x9, x8
        cneg    x10, x10, cc
        mul     x16, x23, x10
        umulh   x10, x23, x10
        cinv    x11, x11, cc
        eor     x16, x16, x11
        eor     x10, x10, x11
        adds    x13, x12, x22
        adc     x22, x22, xzr
        umulh   x23, x7, x9
        adds    x13, x13, x17
        adcs    x22, x22, x23
        adc     x23, x23, xzr
        adds    x22, x22, x17
        adc     x23, x23, xzr
        cmn     x11, #0x1
        adcs    x13, x13, x16
        adcs    x22, x22, x10
        adc     x23, x23, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x22, x22, x22
        adcs    x23, x23, x23
        adc     x19, xzr, xzr
        mul     x10, x6, x6
        mul     x16, x7, x7
        mul     x21, x6, x7
        umulh   x11, x6, x6
        umulh   x17, x7, x7
        umulh   x20, x6, x7
        adds    x11, x11, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x11, x11, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x22, x22, xzr
        adcs    x23, x23, xzr
        adc     x19, x19, xzr
        mul     x14, x8, x8
        mul     x16, x9, x9
        mul     x21, x8, x9
        umulh   x15, x8, x8
        umulh   x17, x9, x9
        umulh   x20, x8, x9
        adds    x15, x15, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x15, x15, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x14, x14, x22
        adcs    x15, x15, x23
        adcs    x16, x16, x19
        adc     x17, x17, xzr
        ldr     x19, [x1, #64]
        add     x23, x19, x19
        mul     x19, x19, x19
        and     x21, x2, #0xfffffffffffff
        mul     x21, x23, x21
        extr    x20, x3, x2, #52
        and     x20, x20, #0xfffffffffffff
        mul     x20, x23, x20
        lsr     x22, x21, #52
        add     x20, x20, x22
        lsl     x21, x21, #12
        extr    x22, x20, x21, #12
        adds    x10, x10, x22
        extr    x21, x4, x3, #40
        and     x21, x21, #0xfffffffffffff
        mul     x21, x23, x21
        lsr     x22, x20, #52
        add     x21, x21, x22
        lsl     x20, x20, #12
        extr    x22, x21, x20, #24
        adcs    x11, x11, x22
        extr    x20, x5, x4, #28
        and     x20, x20, #0xfffffffffffff
        mul     x20, x23, x20
        lsr     x22, x21, #52
        add     x20, x20, x22
        lsl     x21, x21, #12
        extr    x22, x20, x21, #36
        adcs    x12, x12, x22
        extr    x21, x6, x5, #16
        and     x21, x21, #0xfffffffffffff
        mul     x21, x23, x21
        lsr     x22, x20, #52
        add     x21, x21, x22
        lsl     x20, x20, #12
        extr    x22, x21, x20, #48
        adcs    x13, x13, x22
        lsr     x20, x6, #4
        and     x20, x20, #0xfffffffffffff
        mul     x20, x23, x20
        lsr     x22, x21, #52
        add     x20, x20, x22
        lsl     x21, x21, #12
        extr    x24, x20, x21, #60
        extr    x21, x7, x6, #56
        and     x21, x21, #0xfffffffffffff
        mul     x21, x23, x21
        lsr     x22, x20, #52
        add     x21, x21, x22
        lsl     x24, x24, #8
        extr    x22, x21, x24, #8
        adcs    x14, x14, x22
        extr    x20, x8, x7, #44
        and     x20, x20, #0xfffffffffffff
        mul     x20, x23, x20
        lsr     x22, x21, #52
        add     x20, x20, x22
        lsl     x21, x21, #12
        extr    x22, x20, x21, #20
        adcs    x15, x15, x22
        extr    x21, x9, x8, #32
        and     x21, x21, #0xfffffffffffff
        mul     x21, x23, x21
        lsr     x22, x20, #52
        add     x21, x21, x22
        lsl     x20, x20, #12
        extr    x22, x21, x20, #32
        adcs    x16, x16, x22
        lsr     x20, x9, #20
        mul     x20, x23, x20
        lsr     x22, x21, #52
        add     x20, x20, x22
        lsl     x21, x21, #12
        extr    x22, x20, x21, #44
        adcs    x17, x17, x22
        lsr     x20, x20, #44
        adc     x19, x19, x20
        extr    x21, x11, x10, #9
        extr    x20, x12, x11, #9
        stp     x21, x20, [x0]
        extr    x21, x13, x12, #9
        extr    x20, x14, x13, #9
        stp     x21, x20, [x0, #16]
        extr    x21, x15, x14, #9
        extr    x20, x16, x15, #9
        stp     x21, x20, [x0, #32]
        extr    x21, x17, x16, #9
        extr    x20, x19, x17, #9
        stp     x21, x20, [x0, #48]
        and     x22, x10, #0x1ff
        lsr     x19, x19, #9
        add     x22, x22, x19
        str     x22, [x0, #64]
        mul     x12, x2, x4
        mul     x17, x3, x5
        umulh   x22, x2, x4
        subs    x23, x2, x3
        cneg    x23, x23, cc
        csetm   x11, cc
        subs    x10, x5, x4
        cneg    x10, x10, cc
        mul     x16, x23, x10
        umulh   x10, x23, x10
        cinv    x11, x11, cc
        eor     x16, x16, x11
        eor     x10, x10, x11
        adds    x13, x12, x22
        adc     x22, x22, xzr
        umulh   x23, x3, x5
        adds    x13, x13, x17
        adcs    x22, x22, x23
        adc     x23, x23, xzr
        adds    x22, x22, x17
        adc     x23, x23, xzr
        cmn     x11, #0x1
        adcs    x13, x13, x16
        adcs    x22, x22, x10
        adc     x23, x23, x11
        adds    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x22, x22, x22
        adcs    x23, x23, x23
        adc     x19, xzr, xzr
        mul     x10, x2, x2
        mul     x16, x3, x3
        mul     x21, x2, x3
        umulh   x11, x2, x2
        umulh   x17, x3, x3
        umulh   x20, x2, x3
        adds    x11, x11, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x11, x11, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x12, x12, x16
        adcs    x13, x13, x17
        adcs    x22, x22, xzr
        adcs    x23, x23, xzr
        adc     x19, x19, xzr
        mul     x14, x4, x4
        mul     x16, x5, x5
        mul     x21, x4, x5
        umulh   x15, x4, x4
        umulh   x17, x5, x5
        umulh   x20, x4, x5
        adds    x15, x15, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x15, x15, x21
        adcs    x16, x16, x20
        adc     x17, x17, xzr
        adds    x14, x14, x22
        adcs    x15, x15, x23
        adcs    x16, x16, x19
        adc     x17, x17, xzr
        ldp     x21, x20, [x0]
        adds    x21, x21, x10
        adcs    x20, x20, x11
        stp     x21, x20, [x0]
        ldp     x21, x20, [x0, #16]
        adcs    x21, x21, x12
        adcs    x20, x20, x13
        stp     x21, x20, [x0, #16]
        ldp     x21, x20, [x0, #32]
        adcs    x21, x21, x14
        adcs    x20, x20, x15
        stp     x21, x20, [x0, #32]
        ldp     x21, x20, [x0, #48]
        adcs    x21, x21, x16
        adcs    x20, x20, x17
        stp     x21, x20, [x0, #48]
        ldr     x22, [x0, #64]
        adc     x22, x22, xzr
        str     x22, [x0, #64]
        mul     x10, x2, x6
        mul     x14, x3, x7
        mul     x15, x4, x8
        mul     x16, x5, x9
        umulh   x17, x2, x6
        adds    x14, x14, x17
        umulh   x17, x3, x7
        adcs    x15, x15, x17
        umulh   x17, x4, x8
        adcs    x16, x16, x17
        umulh   x17, x5, x9
        adc     x17, x17, xzr
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
        cneg    x22, x22, cc
        csetm   x19, cc
        subs    x20, x9, x8
        cneg    x20, x20, cc
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x15, x15, x21
        eor     x20, x20, x19
        adcs    x16, x16, x20
        adc     x17, x17, x19
        subs    x22, x2, x3
        cneg    x22, x22, cc
        csetm   x19, cc
        subs    x20, x7, x6
        cneg    x20, x20, cc
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc
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
        cneg    x22, x22, cc
        csetm   x19, cc
        subs    x20, x9, x7
        cneg    x20, x20, cc
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x14, x14, x21
        eor     x20, x20, x19
        adcs    x15, x15, x20
        adcs    x16, x16, x19
        adc     x17, x17, x19
        subs    x22, x2, x4
        cneg    x22, x22, cc
        csetm   x19, cc
        subs    x20, x8, x6
        cneg    x20, x20, cc
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc
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
        cneg    x22, x22, cc
        csetm   x19, cc
        subs    x20, x9, x6
        cneg    x20, x20, cc
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x13, x13, x21
        eor     x20, x20, x19
        adcs    x14, x14, x20
        adcs    x15, x15, x19
        adcs    x16, x16, x19
        adc     x17, x17, x19
        subs    x22, x3, x4
        cneg    x22, x22, cc
        csetm   x19, cc
        subs    x20, x8, x7
        cneg    x20, x20, cc
        mul     x21, x22, x20
        umulh   x20, x22, x20
        cinv    x19, x19, cc
        cmn     x19, #0x1
        eor     x21, x21, x19
        adcs    x13, x13, x21
        eor     x20, x20, x19
        adcs    x14, x14, x20
        adcs    x15, x15, x19
        adcs    x16, x16, x19
        adc     x17, x17, x19
        ldp     x21, x20, [x0]
        extr    x2, x15, x14, #8
        adds    x2, x2, x21
        extr    x3, x16, x15, #8
        adcs    x3, x3, x20
        ldp     x21, x20, [x0, #16]
        extr    x4, x17, x16, #8
        adcs    x4, x4, x21
        and     x22, x3, x4
        lsr     x5, x17, #8
        adcs    x5, x5, x20
        and     x22, x22, x5
        ldp     x21, x20, [x0, #32]
        lsl     x6, x10, #1
        adcs    x6, x6, x21
        and     x22, x22, x6
        extr    x7, x11, x10, #63
        adcs    x7, x7, x20
        and     x22, x22, x7
        ldp     x21, x20, [x0, #48]
        extr    x8, x12, x11, #63
        adcs    x8, x8, x21
        and     x22, x22, x8
        extr    x9, x13, x12, #63
        adcs    x9, x9, x20
        and     x22, x22, x9
        ldr     x21, [x0, #64]
        extr    x10, x14, x13, #63
        and     x10, x10, #0x1ff
        adc     x10, x21, x10
        lsr     x20, x10, #9
        orr     x10, x10, #0xfffffffffffffe00
        cmp     xzr, xzr
        adcs    xzr, x2, x20
        adcs    xzr, x22, xzr
        adcs    xzr, x10, xzr
        adcs    x2, x2, x20
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        adcs    x5, x5, xzr
        adcs    x6, x6, xzr
        adcs    x7, x7, xzr
        adcs    x8, x8, xzr
        adcs    x9, x9, xzr
        adc     x10, x10, xzr
        and     x10, x10, #0x1ff
        stp     x2, x3, [x0]
        stp     x4, x5, [x0, #16]
        stp     x6, x7, [x0, #32]
        stp     x8, x9, [x0, #48]
        str     x10, [x0, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jmixadd_local_sqr_p521)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jmixadd_local_sub_p521)

Lp521_jmixadd_local_sub_p521:
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

S2N_BN_SIZE_DIRECTIVE(Lp521_jmixadd_local_sub_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
