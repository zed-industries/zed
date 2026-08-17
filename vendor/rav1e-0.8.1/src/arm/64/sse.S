/* Copyright (c) 2023, The rav1e contributors. All rights reserved
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#include "src/arm/asm.S"
#include "util.S"

.macro LOAD_X4 t=q
    ldr     \t\()0, [x0]
    ldr     \t\()4, [x2]
    ldr     \t\()1, [x0, x1]
    ldr     \t\()5, [x2, x3]
    ldr     \t\()2, [x0, x11]
    ldr     \t\()6, [x2, x12]
    ldr     \t\()3, [x0, x8]
    ldr     \t\()7, [x2, x9]
    ldr     \t\()16, [x4]
    add     x0, x0, x1, lsl 2
    add     x2, x2, x3, lsl 2
    add     x4, x4, x5
    subs    w10, w10, #4
.ifc \t,q
    mov     v18.d[0], v16.d[1]
.else
    mov     v0.\t[1], v1.\t[0]
    mov     v4.\t[1], v5.\t[0]
    mov     v2.\t[1], v3.\t[0]
    mov     v6.\t[1], v7.\t[0]
.endif
.endm

.macro INIT width, height
.if \width <= 16
    add     x11, x1, x1
    add     x12, x3, x3
    add     x8, x1, x1, lsl 1
    add     x9, x3, x3, lsl 1
.elseif \width >= 64
    mov     w8, #(\width)
    sxtw    x9, w8
.endif
    movi    v17.4s,  #0
    mov     w10, #(\height)
.endm

// x0: src: *const u8,
// x1: src_stride: isize,
// x2: dst: *const u8,
// x3: dst_stride: isize,
// x4: scale: *const u32,
// x5: scale_stride: isize,
function weighted_sse_4x4_neon, export=1
    INIT 4, 4
L(wsse_w4):
    LOAD_X4 t=s
    uabd    v0.8b, v0.8b, v4.8b  // diff pixel values
    uabd    v1.8b, v2.8b, v6.8b
    umull   v0.8h, v0.8b, v0.8b  // square
    umull   v1.8h, v1.8b, v1.8b
    uaddl   v2.4s, v0.4h, v1.4h  // accumulate
    uaddl2  v3.4s, v0.8h, v1.8h
    add     v0.4s, v2.4s, v3.4s
    addv    s0, v0.4s
    umull   v0.2d, v0.2s, v16.2s // apply scale
    urshr   d0, d0, #8
    add     v17.2d, v17.2d, v0.2d
    bne     L(wsse_w4)
    fmov    x0, d17
    ret
endfunc

.macro RET_SUM
    mov     v0.d[0], v17.d[1]
    add     d0, d0, d17
    fmov    x0, d0
    ret
.endm

function weighted_sse_8x8_neon, export=1
    INIT 8, 8
L(wsse_w8):
    LOAD_X4 t=d
    uabd    v0.16b, v0.16b, v4.16b  // diff pixel values
    uabd    v1.16b, v2.16b, v6.16b
    umull2  v2.8h, v0.16b, v0.16b   // square
    umull2  v3.8h, v1.16b, v1.16b
    umull   v0.8h, v0.8b, v0.8b
    umull   v1.8h, v1.8b, v1.8b
    uaddlp  v2.4s, v2.8h            // accumulate
    uaddlp  v3.4s, v3.8h
    uaddlp  v0.4s, v0.8h
    uaddlp  v1.4s, v1.8h
    uaddlp  v2.2d, v2.4s
    uadalp  v2.2d, v3.4s
    uadalp  v2.2d, v0.4s
    uadalp  v2.2d, v1.4s
    xtn     v0.2s, v2.2d
    umull   v0.2d, v0.2s, v16.2s    // apply scale
    urshr   v0.2d, v0.2d, #8
    add     v17.2d, v17.2d, v0.2d
    bne     L(wsse_w8)
    RET_SUM
endfunc

function weighted_sse_16x16_neon, export=1
    INIT 16, 16
L(wsse_w16):
    LOAD_X4
    uabd    v0.16b, v0.16b, v4.16b  // diff pixel values
    uabd    v1.16b, v1.16b, v5.16b
    uabd    v2.16b, v2.16b, v6.16b
    uabd    v3.16b, v3.16b, v7.16b
    umull2  v4.8h, v0.16b, v0.16b   // square
    umull2  v5.8h, v1.16b, v1.16b
    umull2  v6.8h, v2.16b, v2.16b
    umull2  v7.8h, v3.16b, v3.16b
    umull   v0.8h, v0.8b, v0.8b
    umull   v1.8h, v1.8b, v1.8b
    umull   v2.8h, v2.8b, v2.8b
    umull   v3.8h, v3.8b, v3.8b
    uaddlp  v4.4s, v4.8h            // accumulate
    uaddlp  v5.4s, v5.8h
    uaddlp  v6.4s, v6.8h
    uaddlp  v7.4s, v7.8h
    uaddlp  v0.4s, v0.8h
    uaddlp  v1.4s, v1.8h
    uaddlp  v2.4s, v2.8h
    uaddlp  v3.4s, v3.8h
    uaddlp  v4.2d, v4.4s
    uadalp  v4.2d, v5.4s
    uadalp  v4.2d, v6.4s
    uadalp  v4.2d, v7.4s
    xtn     v4.2s, v4.2d
    uaddlp  v0.2d, v0.4s
    uadalp  v0.2d, v1.4s
    uadalp  v0.2d, v2.4s
    uadalp  v0.2d, v3.4s
    xtn     v0.2s, v0.2d
    umull   v4.2d, v4.2s, v18.2s    // apply scale
    umull   v0.2d, v0.2s, v16.2s
    urshr   v4.2d, v4.2d, #8
    urshr   v0.2d, v0.2d, #8
    add     v17.2d, v17.2d, v4.2d
    add     v17.2d, v17.2d, v0.2d
    bne     L(wsse_w16)
    RET_SUM
endfunc

.macro LOAD_32X4 vert=1, hbd=0
    ldp     q0, q22, [x0]
    ldp     q4, q26, [x2]
    add     x0, x0, x1
    add     x2, x2, x3
    ldp     q1, q23, [x0]
    ldp     q5, q27, [x2]
    add     x0, x0, x1
    add     x2, x2, x3
    ldp     q2, q24, [x0]
    ldp     q6, q28, [x2]
    add     x0, x0, x1
    add     x2, x2, x3
    ldp     q3, q25, [x0]
    ldp     q7, q29, [x2]
    add     x0, x0, x1
    add     x2, x2, x3
.if \vert == 1
.if \hbd == 0
    ldp     q16, q19, [x4]
.else
    ldr     q16, [x4]
.endif
    add     x4, x4, x5
    subs    w10, w10, #4
.else
    sub     x0, x0, x1, lsl 2
    sub     x2, x2, x3, lsl 2
    add     x0, x0, #32
    add     x2, x2, #32
.if \hbd == 0
    ldp     q16, q19, [x4]
    add     x4, x4, #32
.else
    ldr     q16, [x4]
    add     x4, x4, #16
.endif
.endif
    mov     v18.d[0], v16.d[1]
.if \hbd == 0
    mov     v20.d[0], v19.d[1]
.endif
.endm

.macro WEIGHTED_SSE_32X4
    uabd    v0.16b, v0.16b, v4.16b     // diff pixel values
    uabd    v1.16b, v1.16b, v5.16b
    uabd    v2.16b, v2.16b, v6.16b
    uabd    v3.16b, v3.16b, v7.16b
    uabd    v22.16b, v22.16b, v26.16b
    uabd    v23.16b, v23.16b, v27.16b
    uabd    v24.16b, v24.16b, v28.16b
    uabd    v25.16b, v25.16b, v29.16b
    umull2  v4.8h, v0.16b, v0.16b      // square
    umull2  v5.8h, v1.16b, v1.16b
    umull2  v6.8h, v2.16b, v2.16b
    umull2  v7.8h, v3.16b, v3.16b
    umull2  v26.8h, v22.16b, v22.16b
    umull2  v27.8h, v23.16b, v23.16b
    umull2  v28.8h, v24.16b, v24.16b
    umull2  v29.8h, v25.16b, v25.16b
    umull   v0.8h, v0.8b, v0.8b
    umull   v1.8h, v1.8b, v1.8b
    umull   v2.8h, v2.8b, v2.8b
    umull   v3.8h, v3.8b, v3.8b
    umull   v22.8h, v22.8b, v22.8b
    umull   v23.8h, v23.8b, v23.8b
    umull   v24.8h, v24.8b, v24.8b
    umull   v25.8h, v25.8b, v25.8b
    uaddlp  v4.4s, v4.8h               // accumulate
    uadalp  v4.4s, v5.8h
    uadalp  v4.4s, v6.8h
    uadalp  v4.4s, v7.8h
    uaddlp  v26.4s, v26.8h
    uadalp  v26.4s, v27.8h
    uadalp  v26.4s, v28.8h
    uadalp  v26.4s, v29.8h
    uaddlp  v0.4s, v0.8h
    uadalp  v0.4s, v1.8h
    uadalp  v0.4s, v2.8h
    uadalp  v0.4s, v3.8h
    uaddlp  v22.4s, v22.8h
    uadalp  v22.4s, v23.8h
    uadalp  v22.4s, v24.8h
    uadalp  v22.4s, v25.8h
    uaddlp  v4.2d, v4.4s
    uaddlp  v26.2d, v26.4s
    uaddlp  v0.2d, v0.4s
    uaddlp  v22.2d, v22.4s
    xtn     v4.2s, v4.2d
    xtn     v26.2s, v26.2d
    xtn     v0.2s, v0.2d
    xtn     v22.2s, v22.2d
    umull   v4.2d, v4.2s, v18.2s       // apply scale
    umull   v26.2d, v26.2s, v20.2s
    umull   v0.2d, v0.2s, v16.2s
    umull   v22.2d, v22.2s, v19.2s
    urshr   v4.2d, v4.2d, #8
    urshr   v26.2d, v26.2d, #8
    urshr   v0.2d, v0.2d, #8
    urshr   v22.2d, v22.2d, #8
    add     v4.2d, v4.2d, v26.2d
    add     v0.2d, v0.2d, v22.2d
    add     v17.2d, v17.2d, v4.2d
    add     v17.2d, v17.2d, v0.2d
.endm

function weighted_sse_32x32_neon, export=1
    INIT 32, 32
L(wsse_w32):
    LOAD_32X4
    WEIGHTED_SSE_32X4
    bne     L(wsse_w32)
    RET_SUM
endfunc

function weighted_sse_64x64_neon, export=1
    INIT 64, 64
L(wsse_w32up):
    LOAD_32X4 vert=0
    WEIGHTED_SSE_32X4
    subs    w8, w8, #32
    bne     L(wsse_w32up)
    mov     w8, w9
    sub     x0, x0, x9
    sub     x2, x2, x9
    add     x0, x0, x1, lsl 2
    add     x2, x2, x3, lsl 2
    sub     x4, x4, x9
    add     x4, x4, x5
    subs    w10, w10, #4
    bne     L(wsse_w32up)
    RET_SUM
endfunc

.macro weighted_sse width, height
function weighted_sse_\width\()x\height\()_neon, export=1
    INIT \width, \height
.if \width <= 32
    b       L(wsse_w\width)
.else
    b       L(wsse_w32up)
.endif
endfunc
.endm

weighted_sse 4, 8
weighted_sse 4, 16
weighted_sse 8, 4
weighted_sse 8, 16
weighted_sse 8, 32
weighted_sse 16, 4
weighted_sse 16, 8
weighted_sse 16, 32
weighted_sse 16, 64
weighted_sse 32, 8
weighted_sse 32, 16
weighted_sse 32, 64
weighted_sse 64, 16
weighted_sse 64, 32
weighted_sse 64, 128
weighted_sse 128, 64
weighted_sse 128, 128

.macro LOAD_X4_HBD t=q
    ldr     \t\()0, [x0]
    ldr     \t\()4, [x2]
    ldr     \t\()1, [x0, x1]
    ldr     \t\()5, [x2, x3]
    ldr     \t\()2, [x0, x11]
    ldr     \t\()6, [x2, x12]
    ldr     \t\()3, [x0, x8]
    ldr     \t\()7, [x2, x9]
.ifc \t,q
    ldr     d16, [x4]
.else
    ldr     s16, [x4]
.endif
    add     x0, x0, x1, lsl 2
    add     x2, x2, x3, lsl 2
    add     x4, x4, x5
    subs    w10, w10, #4
.endm

.macro INIT_HBD width, height
.if \width <= 8
    add     x11, x1, x1
    add     x12, x3, x3
    add     x8, x1, x1, lsl 1
    add     x9, x3, x3, lsl 1
.elseif \width >= 32
    mov     w8, #(\width)
    sxtw    x9, w8
.endif
    movi    v17.4s,  #0
    mov     w10, #(\height)
.endm

// x0: src: *const u16,
// x1: src_stride: isize,
// x2: dst: *const u16,
// x3: dst_stride: isize,
// x4: scale: *const u32,
// x5: scale_stride: isize,
function weighted_sse_4x4_hbd_neon, export=1
    INIT_HBD 4, 4
L(wsse_hbd_w4):
    LOAD_X4_HBD t=d
    uabd    v0.8h, v0.8h, v4.8h  // diff pixel values
    uabd    v1.8h, v1.8h, v5.8h
    uabd    v2.8h, v2.8h, v6.8h
    uabd    v3.8h, v3.8h, v7.8h
    umull   v0.4s, v0.4h, v0.4h  // square
    umull   v1.4s, v1.4h, v1.4h
    umull   v2.4s, v2.4h, v2.4h
    umull   v3.4s, v3.4h, v3.4h
    add     v0.4s, v0.4s, v1.4s  // accumulate
    add     v2.4s, v2.4s, v3.4s
    add     v0.4s, v0.4s, v2.4s
    addv    s0, v0.4s
    umull   v0.2d, v0.2s, v16.2s // apply scale
    urshr   d0, d0, #8
    add     v17.2d, v17.2d, v0.2d
    bne     L(wsse_hbd_w4)
    fmov    x0, d17
    ret
endfunc

function weighted_sse_8x8_hbd_neon, export=1
    INIT_HBD 8, 8
L(wsse_hbd_w8):
    LOAD_X4_HBD
    uabd    v4.8h, v0.8h, v4.8h  // diff pixel values
    uabd    v5.8h, v1.8h, v5.8h
    uabd    v6.8h, v2.8h, v6.8h
    uabd    v7.8h, v3.8h, v7.8h
    umull   v0.4s, v4.4h, v4.4h   // square
    umull   v1.4s, v5.4h, v5.4h
    umull   v2.4s, v6.4h, v6.4h
    umull   v3.4s, v7.4h, v7.4h
    umull2  v4.4s, v4.8h, v4.8h
    umull2  v5.4s, v5.8h, v5.8h
    umull2  v6.4s, v6.8h, v6.8h
    umull2  v7.4s, v7.8h, v7.8h
    add     v0.4s, v0.4s, v1.4s  // accumulate
    add     v2.4s, v2.4s, v3.4s
    add     v4.4s, v4.4s, v5.4s
    add     v6.4s, v6.4s, v7.4s
    add     v0.4s, v0.4s, v2.4s
    add     v4.4s, v4.4s, v6.4s
    addv    s0, v0.4s
    addv    s4, v4.4s
    mov     v0.s[1], v4.s[0]
    umull   v0.2d, v0.2s, v16.2s    // apply scale
    urshr   v0.2d, v0.2d, #8
    add     v17.2d, v17.2d, v0.2d
    bne     L(wsse_hbd_w8)
    RET_SUM
endfunc

.macro WEIGHTED_SSE_16X4_HBD
    uabd    v0.8h, v0.8h, v4.8h     // diff pixel values
    uabd    v1.8h, v1.8h, v5.8h
    uabd    v2.8h, v2.8h, v6.8h
    uabd    v3.8h, v3.8h, v7.8h
    uabd    v22.8h, v22.8h, v26.8h
    uabd    v23.8h, v23.8h, v27.8h
    uabd    v24.8h, v24.8h, v28.8h
    uabd    v25.8h, v25.8h, v29.8h
    umull2  v4.4s, v0.8h, v0.8h      // square
    umull2  v5.4s, v1.8h, v1.8h
    umull2  v6.4s, v2.8h, v2.8h
    umull2  v7.4s, v3.8h, v3.8h
    umull2  v26.4s, v22.8h, v22.8h
    umull2  v27.4s, v23.8h, v23.8h
    umull2  v28.4s, v24.8h, v24.8h
    umull2  v29.4s, v25.8h, v25.8h
    umull   v0.4s, v0.4h, v0.4h
    umull   v1.4s, v1.4h, v1.4h
    umull   v2.4s, v2.4h, v2.4h
    umull   v3.4s, v3.4h, v3.4h
    umull   v22.4s, v22.4h, v22.4h
    umull   v23.4s, v23.4h, v23.4h
    umull   v24.4s, v24.4h, v24.4h
    umull   v25.4s, v25.4h, v25.4h
    add     v0.4s, v0.4s, v1.4s      // accumulate
    add     v2.4s, v2.4s, v3.4s
    add     v4.4s, v4.4s, v5.4s
    add     v6.4s, v6.4s, v7.4s
    add     v22.4s, v22.4s, v23.4s
    add     v24.4s, v24.4s, v25.4s
    add     v26.4s, v26.4s, v27.4s
    add     v28.4s, v28.4s, v29.4s
    add     v0.4s, v0.4s, v2.4s
    add     v4.4s, v4.4s, v6.4s
    add     v22.4s, v22.4s, v24.4s
    add     v26.4s, v26.4s, v28.4s
    addv    s0, v0.4s
    addv    s4, v4.4s
    addv    s22, v22.4s
    addv    s26, v26.4s
    mov     v0.s[1], v4.s[0]
    mov     v22.s[1], v26.s[0]
    umull   v0.2d, v0.2s, v16.2s     // apply scale
    umull   v22.2d, v22.2s, v18.2s
    urshr   v0.2d, v0.2d, #8
    urshr   v22.2d, v22.2d, #8
    add     v0.2d, v0.2d, v22.2d
    add     v17.2d, v17.2d, v0.2d
.endm

function weighted_sse_16x16_hbd_neon, export=1
    INIT_HBD 16, 16
L(wsse_hbd_w16):
    LOAD_32X4 vert=1, hbd=1
    WEIGHTED_SSE_16X4_HBD
    bne     L(wsse_hbd_w16)
    RET_SUM
endfunc

function weighted_sse_32x32_hbd_neon, export=1
    INIT_HBD 32, 32
L(wsse_hbd_w32up):
    LOAD_32X4 vert=0, hbd=1
    WEIGHTED_SSE_16X4_HBD
    subs    w8, w8, #16
    bne     L(wsse_hbd_w32up)
    mov     w8, w9
    sub     x0, x0, x9, lsl 1
    sub     x2, x2, x9, lsl 1
    add     x0, x0, x1, lsl 2
    add     x2, x2, x3, lsl 2
    sub     x4, x4, x9
    add     x4, x4, x5
    subs    w10, w10, #4
    bne     L(wsse_hbd_w32up)
    RET_SUM
endfunc

.macro weighted_sse_hbd width, height
function weighted_sse_\width\()x\height\()_hbd_neon, export=1
    INIT_HBD \width, \height
.if \width <= 16
    b       L(wsse_hbd_w\width)
.else
    b       L(wsse_hbd_w32up)
.endif
endfunc
.endm

weighted_sse_hbd 4, 8
weighted_sse_hbd 4, 16
weighted_sse_hbd 8, 4
weighted_sse_hbd 8, 16
weighted_sse_hbd 8, 32
weighted_sse_hbd 16, 4
weighted_sse_hbd 16, 8
weighted_sse_hbd 16, 32
weighted_sse_hbd 16, 64
weighted_sse_hbd 32, 8
weighted_sse_hbd 32, 16
weighted_sse_hbd 32, 64
weighted_sse_hbd 64, 16
weighted_sse_hbd 64, 32
weighted_sse_hbd 64, 64
weighted_sse_hbd 64, 128
weighted_sse_hbd 128, 64
weighted_sse_hbd 128, 128
