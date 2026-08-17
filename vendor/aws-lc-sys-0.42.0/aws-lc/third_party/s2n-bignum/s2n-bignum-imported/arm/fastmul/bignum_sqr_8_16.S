// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[8]; output z[16]
//
//    extern void bignum_sqr_8_16(uint64_t z[static 16], const uint64_t x[static 8]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_8_16)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_8_16)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_8_16)
        .text
        .balign 4


S2N_BN_SYMBOL(bignum_sqr_8_16):
        CFI_START

// Save registers

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)

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

        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_8_16)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
