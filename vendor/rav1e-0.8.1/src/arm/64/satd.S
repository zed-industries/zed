/* Copyright (c) 2022-2023, The rav1e contributors. All rights reserved
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

.macro butterfly r0, r1, r2, r3, t=8h
    add  \r0\().\t, \r2\().\t, \r3\().\t
    sub  \r1\().\t, \r2\().\t, \r3\().\t
.endm

.macro butterflyw r0, r1, r2, r3, r4, r5
    sxtl    \r0\().4s, \r4\().4h
    sxtl2   \r2\().4s, \r4\().8h
    ssubw   \r1\().4s, \r0\().4s, \r5\().4h
    ssubw2  \r3\().4s, \r2\().4s, \r5\().8h
    saddw   \r0\().4s, \r0\().4s, \r5\().4h
    saddw2  \r2\().4s, \r2\().4s, \r5\().8h
.endm

.macro interleave r0, r1, r2, r3
    zip1 \r0\().8h, \r2\().8h, \r3\().8h
    zip2 \r1\().8h, \r2\().8h, \r3\().8h
.endm

.macro interleave_pairs r0, r1, r2, r3
    zip1 \r0\().4s, \r2\().4s, \r3\().4s
    zip2 \r1\().4s, \r2\().4s, \r3\().4s
.endm

.macro interleave_quads r0, r1, r2, r3
    zip1 \r0\().2d, \r2\().2d, \r3\().2d
    zip2 \r1\().2d, \r2\().2d, \r3\().2d
.endm

.macro normalize_4
    add     w0, w0, 2
    lsr     w0, w0, 2
.endm

.macro normalize_8
    add     w0, w0, 4
    lsr     w0, w0, 3
.endm

// x0: src: *const u8,
// x1: src_stride: isize,
// x2: dst: *const u8,
// x3: dst_stride: isize,
function satd4x4_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    ldr     s0, [src]
    ldr     s1, [dst]

    // subtract; cast to 16-bit
    usubl v0.8h, v0.8b, v1.8b

    ldr     s1, [src, src_stride]
    ldr     s2, [dst, dst_stride]

    usubl v1.8h, v1.8b, v2.8b

    // stride * 2
    lsl     x8, src_stride, 1
    lsl     x9, dst_stride, 1

    ldr     s2, [src, x8]
    ldr     s3, [dst, x9]

    usubl v2.8h, v2.8b, v3.8b

    // stride * 3
    add     x8, src_stride, src_stride, lsl 1
    add     x9, dst_stride, dst_stride, lsl 1

    ldr     s3, [src, x8]
    ldr     s4, [dst, x9]

    usubl v3.8h, v3.8b, v4.8b

    // pack rows 0-2, 1-3
    mov   v0.d[1], v2.d[0]
    mov   v1.d[1], v3.d[0]

    // Horizontal transform

    // v0    0 1 2 3   8  9 10 11
    // v1    4 5 6 7  12 13 14 15

    butterfly v2, v3, v0, v1

    // v2    [0+4][1+5][2+6][3+7] [8+12][9+13][10+14][11+15]
    // v3    [0-4][1-5][2-6][3-7] [8-12][9-13][10-14][11-15]

    interleave v0, v1, v2, v3

    // v0    [ 0+4][ 0-4][ 1+5][ 1-5] [2 + 6][2 - 6][3 + 7][3 - 7]
    // v1    [8+12][8-12][9+13][9-13] [10+14][10-14][11+15][11-15]

    butterfly v2, v3, v0, v1

    // v2    [0+4+8+12][0-4+8-12][1+5+9+13][1-5+9-13] [2+6+10+14][2-6+10-14][3+7+11+15][3-7+11-15]
    // v3    [0+4-8-12][0-4-8+12][1+5-9-13][1-5-9+13] [2+6-10-14][2-6-10+14][3+7-11-15][3-7-11+15]

    interleave_pairs v0, v1, v2, v3

    // Vertical transform

    butterfly v2, v3, v0, v1

    interleave v0, v1, v2, v3

    butterfly v2, v3, v0, v1

    // sum up transform
    abs   v2.8h, v2.8h
    abs   v3.8h, v3.8h

    add v0.8h, v2.8h, v3.8h

    addv    h0, v0.8h
    fmov    w0, s0
    normalize_4
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride
endfunc

.macro DOUBLE_HADAMARD_4X4 hbd=0
    // Horizontal transform

    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5

    interleave v0, v1, v2, v3
    interleave v4, v5, v6, v7

    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5

    interleave_pairs v0, v1, v2, v3
    interleave_pairs v4, v5, v6, v7

    // Vertical transform

    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5

    interleave v0, v1, v2, v3
    interleave v4, v5, v6, v7

.if \hbd == 0
    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5
.else
    butterflyw v2, v3, v16, v17, v0, v1
    butterflyw v6, v7, v18, v19, v4, v5
.endif
.endm

.macro SUM_DOUBLE_HADAMARD_4X4
    abs     v2.8h, v2.8h
    abs     v3.8h, v3.8h
    abs     v6.8h, v6.8h
    abs     v7.8h, v7.8h

    add     v0.8h, v2.8h, v3.8h
    add     v1.8h, v6.8h, v7.8h
    add     v0.8h, v0.8h, v1.8h

    addv    h0, v0.8h
    fmov    w0, s0
    normalize_4
.endm

function satd8x4_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    // load 8 pixel row
    ldr     d0, [src]
    ldr     d1, [dst]

    usubl v0.8h, v0.8b, v1.8b

    ldr     d1, [src, src_stride]
    ldr     d2, [dst, dst_stride]

    usubl v1.8h, v1.8b, v2.8b

    lsl     x8, src_stride, 1
    lsl     x9, dst_stride, 1

    ldr     d2, [src, x8]
    ldr     d3, [dst, x9]

    usubl v2.8h, v2.8b, v3.8b

    // stride * 3
    add     x8, src_stride, src_stride, lsl 1
    add     x9, dst_stride, dst_stride, lsl 1

    ldr     d3, [src, x8]
    ldr     d4, [dst, x9]

    usubl v3.8h, v3.8b, v4.8b

    // extract top 64 bits out of register
    // (4 x 16 bits = 64)

    ext v4.16b, v0.16b, v0.16b, 8
    ext v5.16b, v1.16b, v1.16b, 8

    // pack rows 0-2, 1-3 (set 1)
    mov   v0.d[1], v2.d[0]
    mov   v1.d[1], v3.d[0]

    // pack rows 0-2, 1-3 (set 2)
    mov   v4.d[1], v2.d[1]
    mov   v5.d[1], v3.d[1]

    // v2-3 temp registers for first 4x4 block//
    // 6-7 for second block

    DOUBLE_HADAMARD_4X4

    SUM_DOUBLE_HADAMARD_4X4
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride
endfunc

.macro load_row n0, n1, src, dst, src_stride, dst_stride, should_add=1
    ldr     s\n0, [\src]
    ldr     s\n1, [\dst]

    usubl v\n0\().8h, v\n0\().8b, v\n1\().8b

.if \should_add != 0
    add     \src, \src, \src_stride
    add     \dst, \dst, \dst_stride
.endif
.endm

.macro load_row2 n0, n1, src, dst, src_stride, dst_stride
    ldr     s\n0, [\src, \src_stride]
    ldr     s\n1, [\dst, \dst_stride]

    usubl v\n0\().8h, v\n0\().8b, v\n1\().8b
.endm

function satd4x8_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    // 0 * stride
    load_row    0, 1, src, dst, src_stride, dst_stride, 0
    // 1 * stride
    load_row2   1, 2, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    // pattern repeats
    load_row    2, 3, src, dst, src_stride, dst_stride, 0
    load_row2   3, 4, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    4, 5, src, dst, src_stride, dst_stride, 0
    load_row2   5, 6, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    6, 7, src, dst, src_stride, dst_stride, 0
    load_row2   7, 8, src, dst, src_stride, dst_stride

    // pack rows
    mov   v0.d[1], v2.d[0]
    mov   v1.d[1], v3.d[0]

    mov   v4.d[1], v6.d[0]
    mov   v5.d[1], v7.d[0]

    DOUBLE_HADAMARD_4X4

    SUM_DOUBLE_HADAMARD_4X4
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride
endfunc

function satd16x4_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    #define ROW1 v0
    #define ROW2 v1
    #define TMP1 v2
    #define TMP2 v3

    #define ROW3 v4
    #define ROW4 v5
    #define TMP3 v6
    #define TMP4 v7

    #define ROW5 v16
    #define ROW6 v17
    #define TMP5 v20
    #define TMP6 v21

    #define ROW7 v18
    #define ROW8 v19
    #define TMP7 v22
    #define TMP8 v23

    // load 16 pixel row
    ldr     q0, [src]
    ldr     q1, [dst]

    usubl2  v16.8h, v0.16b, v1.16b
    usubl   v0.8h, v0.8b, v1.8b

    ldr     q1, [src, src_stride]
    ldr     q2, [dst, dst_stride]

    usubl2  v17.8h, v1.16b, v2.16b
    usubl   v1.8h, v1.8b, v2.8b

    lsl     x8, src_stride, 1
    lsl     x9, dst_stride, 1

    ldr     q2, [src, x8]
    ldr     q3, [dst, x9]

    usubl2  v6.8h, v2.16b, v3.16b
    usubl   v2.8h, v2.8b, v3.8b

    // stride * 3
    add     x8, src_stride, src_stride, lsl 1
    add     x9, dst_stride, dst_stride, lsl 1

    ldr     q3, [src, x8]
    ldr     q4, [dst, x9]

    usubl2  v7.8h, v3.16b, v4.16b
    usubl   v3.8h, v3.8b, v4.8b

    // swap high/low 64 bits
    ext v4.16b, v0.16b, v0.16b, 8
    ext v5.16b, v1.16b, v1.16b, 8

    mov   v0.d[1], v2.d[0]
    mov   v1.d[1], v3.d[0]

    ext v18.16b, v16.16b, v16.16b, 8
    ext v19.16b, v17.16b, v17.16b, 8

    mov   v16.d[1], v6.d[0]
    mov   v17.d[1], v7.d[0]
    // 2-3 free

    mov   v4.d[1], v2.d[1]
    mov   v5.d[1], v3.d[1]
    // 6-7 free
    mov   v18.d[1], v6.d[1]
    mov   v19.d[1], v7.d[1]

    // 0,1       2,3
    // 4,5       6,7
    // 16,17     20,21
    // 18,19     22,23

    // quadruple 4x4 hadamard

    butterfly TMP1, TMP2, ROW1, ROW2
    butterfly TMP3, TMP4, ROW3, ROW4
    butterfly TMP5, TMP6, ROW5, ROW6
    butterfly TMP7, TMP8, ROW7, ROW8

    interleave ROW1, ROW2, TMP1, TMP2
    interleave ROW3, ROW4, TMP3, TMP4
    interleave ROW5, ROW6, TMP5, TMP6
    interleave ROW7, ROW8, TMP7, TMP8

    butterfly TMP1, TMP2, ROW1, ROW2
    butterfly TMP3, TMP4, ROW3, ROW4
    butterfly TMP5, TMP6, ROW5, ROW6
    butterfly TMP7, TMP8, ROW7, ROW8

    interleave_pairs ROW1, ROW2, TMP1, TMP2
    interleave_pairs ROW3, ROW4, TMP3, TMP4
    interleave_pairs ROW5, ROW6, TMP5, TMP6
    interleave_pairs ROW7, ROW8, TMP7, TMP8

    butterfly TMP1, TMP2, ROW1, ROW2
    butterfly TMP3, TMP4, ROW3, ROW4
    butterfly TMP5, TMP6, ROW5, ROW6
    butterfly TMP7, TMP8, ROW7, ROW8

    interleave ROW1, ROW2, TMP1, TMP2
    interleave ROW3, ROW4, TMP3, TMP4
    interleave ROW5, ROW6, TMP5, TMP6
    interleave ROW7, ROW8, TMP7, TMP8

    butterfly TMP1, TMP2, ROW1, ROW2
    butterfly TMP3, TMP4, ROW3, ROW4
    butterfly TMP5, TMP6, ROW5, ROW6
    butterfly TMP7, TMP8, ROW7, ROW8

    // absolute value of transform coefficients
    abs  TMP1.8h, TMP1.8h
    abs  TMP2.8h, TMP2.8h
    abs  TMP3.8h, TMP3.8h
    abs  TMP4.8h, TMP4.8h
    abs  TMP5.8h, TMP5.8h
    abs  TMP6.8h, TMP6.8h
    abs  TMP7.8h, TMP7.8h
    abs  TMP8.8h, TMP8.8h

    // stage 1 sum
    add TMP1.8h, TMP1.8h, TMP5.8h
    add TMP2.8h, TMP2.8h, TMP6.8h
    add TMP3.8h, TMP3.8h, TMP7.8h
    add TMP4.8h, TMP4.8h, TMP8.8h

    // stage 2 sum
    add TMP1.8h, TMP1.8h, TMP3.8h
    add TMP2.8h, TMP2.8h, TMP4.8h
    add v0.8h, TMP1.8h, TMP2.8h

    addv    h0, v0.8h
    fmov    w0, s0
    normalize_4
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride

    #undef ROW1
    #undef TMP1
    #undef ROW2
    #undef TMP2
    #undef ROW3
    #undef TMP3
    #undef ROW4
    #undef TMP4
    #undef ROW5
    #undef TMP5
    #undef ROW6
    #undef TMP6
    #undef ROW7
    #undef TMP7
    #undef ROW8
    #undef TMP8
endfunc

function satd4x16_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    load_row    0, 1, src, dst, src_stride, dst_stride, 0
    load_row2   1, 2, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    2, 3, src, dst, src_stride, dst_stride, 0
    load_row2   3, 4, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    4, 5, src, dst, src_stride, dst_stride, 0
    load_row2   5, 6, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    6, 7, src, dst, src_stride, dst_stride, 0
    load_row2   7, 16, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    16, 17, src, dst, src_stride, dst_stride, 0
    load_row2   17, 18, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    18, 19, src, dst, src_stride, dst_stride, 0
    load_row2   19, 20, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    20, 21, src, dst, src_stride, dst_stride, 0
    load_row2   21, 22, src, dst, src_stride, dst_stride
    add         src, src, src_stride, lsl 1
    add         dst, dst, dst_stride, lsl 1

    load_row    22, 23, src, dst, src_stride, dst_stride, 0
    load_row2   23, 24, src, dst, src_stride, dst_stride

    // pack rows
    mov   v0.d[1], v2.d[0]
    mov   v1.d[1], v3.d[0]

    mov   v4.d[1], v6.d[0]
    mov   v5.d[1], v7.d[0]

    mov   v16.d[1], v18.d[0]
    mov   v17.d[1], v19.d[0]

    mov   v20.d[1], v22.d[0]
    mov   v21.d[1], v23.d[0]

    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5
    butterfly v18, v19, v16, v17
    butterfly v22, v23, v20, v21

    interleave v0, v1, v2, v3
    interleave v4, v5, v6, v7
    interleave v16, v17, v18, v19
    interleave v20, v21, v22, v23

    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5
    butterfly v18, v19, v16, v17
    butterfly v22, v23, v20, v21

    interleave_pairs v0, v1, v2, v3
    interleave_pairs v4, v5, v6, v7
    interleave_pairs v16, v17, v18, v19
    interleave_pairs v20, v21, v22, v23

    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5
    butterfly v18, v19, v16, v17
    butterfly v22, v23, v20, v21

    interleave v0, v1, v2, v3
    interleave v4, v5, v6, v7
    interleave v16, v17, v18, v19
    interleave v20, v21, v22, v23

    butterfly v2, v3, v0, v1
    butterfly v6, v7, v4, v5
    butterfly v18, v19, v16, v17
    butterfly v22, v23, v20, v21

    abs  v2.8h, v2.8h
    abs  v3.8h, v3.8h
    abs  v6.8h, v6.8h
    abs  v7.8h, v7.8h
    abs  v18.8h, v18.8h
    abs  v19.8h, v19.8h
    abs  v22.8h, v22.8h
    abs  v23.8h, v23.8h

    add v2.8h, v2.8h, v3.8h
    add v6.8h, v6.8h, v7.8h
    add v18.8h, v18.8h, v19.8h
    add v22.8h, v22.8h, v23.8h

    add v2.8h, v2.8h, v6.8h
    add v18.8h, v18.8h, v22.8h
    add v0.8h, v2.8h, v18.8h

    addv    h0, v0.8h
    fmov    w0, s0
    normalize_4
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride
endfunc

.macro load_rows n0, n1, n2, src, dst, src_stride, dst_stride, n3=0, n4=0
.if \n3 == 0
    ldr     d\n0, [\src]
    ldr     d\n1, [\dst]
.else
    ldr     q\n0, [\src]
    ldr     q\n1, [\dst]
    usubl2  v\n3\().8h, v\n0\().16b, v\n1\().16b
.endif
    usubl   v\n0\().8h, v\n0\().8b, v\n1\().8b

.if \n4 == 0
    ldr     d\n1, [\src, \src_stride]
    ldr     d\n2, [\dst, \dst_stride]
.else
    ldr     q\n1, [\src, \src_stride]
    ldr     q\n2, [\dst, \dst_stride]
    usubl2  v\n4\().8h, v\n1\().16b, v\n2\().16b
.endif
    usubl   v\n1\().8h, v\n1\().8b, v\n2\().8b

    add     \src, \src, \src_stride, lsl 1
    add     \dst, \dst, \dst_stride, lsl 1
.endm

.macro HADAMARD_8X8_H \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7

    // Horizontal transform

    butterfly v\b0, v\b1, v\a0, v\a1
    butterfly v\b2, v\b3, v\a2, v\a3
    butterfly v\b4, v\b5, v\a4, v\a5
    butterfly v\b6, v\b7, v\a6, v\a7

    interleave v\a0, v\a1, v\b0, v\b1
    interleave v\a2, v\a3, v\b2, v\b3
    interleave v\a4, v\a5, v\b4, v\b5
    interleave v\a6, v\a7, v\b6, v\b7

    butterfly v\b0, v\b2, v\a0, v\a2
    butterfly v\b1, v\b3, v\a1, v\a3
    butterfly v\b4, v\b6, v\a4, v\a6
    butterfly v\b5, v\b7, v\a5, v\a7

    interleave_pairs v\a0, v\a2, v\b0, v\b2
    interleave_pairs v\a1, v\a3, v\b1, v\b3
    interleave_pairs v\a4, v\a6, v\b4, v\b6
    interleave_pairs v\a5, v\a7, v\b5, v\b7

    butterfly v\b0, v\b4, v\a0, v\a4
    butterfly v\b1, v\b5, v\a1, v\a5
    butterfly v\b2, v\b6, v\a2, v\a6
    butterfly v\b3, v\b7, v\a3, v\a7

    interleave_quads v\a0, v\a4, v\b0, v\b4
    interleave_quads v\a1, v\a5, v\b1, v\b5
    interleave_quads v\a2, v\a6, v\b2, v\b6
    interleave_quads v\a3, v\a7, v\b3, v\b7
.endm

.macro HADAMARD_8X8_V \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7

    // Vertical transform

    butterfly v\b0, v\b1, v\a0, v\a1
    butterfly v\b2, v\b3, v\a2, v\a3
    butterfly v\b4, v\b5, v\a4, v\a5
    butterfly v\b6, v\b7, v\a6, v\a7

    butterfly v\a0, v\a2, v\b0, v\b2
    butterfly v\a1, v\a3, v\b1, v\b3
    butterfly v\a4, v\a6, v\b4, v\b6
    butterfly v\a5, v\a7, v\b5, v\b7

    butterfly v\b0, v\b4, v\a0, v\a4
    butterfly v\b1, v\b5, v\a1, v\a5
    butterfly v\b2, v\b6, v\a2, v\a6
    butterfly v\b3, v\b7, v\a3, v\a7
.endm

.macro SUM_HADAMARD_8X8 \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7

    // absolute value of transform coefficients
    abs  v\b0\().8h, v\b0\().8h
    abs  v\b1\().8h, v\b1\().8h
    abs  v\b2\().8h, v\b2\().8h
    abs  v\b3\().8h, v\b3\().8h
    abs  v\b4\().8h, v\b4\().8h
    abs  v\b5\().8h, v\b5\().8h
    abs  v\b6\().8h, v\b6\().8h
    abs  v\b7\().8h, v\b7\().8h

    // stage 1 sum
    sxtl v\a0\().4s, v\b0\().4h
    sxtl v\a1\().4s, v\b1\().4h
    sxtl v\a2\().4s, v\b2\().4h
    sxtl v\a3\().4s, v\b3\().4h
    saddw2  v\a0\().4s, v\a0\().4s, v\b0\().8h
    saddw2  v\a1\().4s, v\a1\().4s, v\b1\().8h
    saddw2  v\a2\().4s, v\a2\().4s, v\b2\().8h
    saddw2  v\a3\().4s, v\a3\().4s, v\b3\().8h
    saddw   v\a0\().4s, v\a0\().4s, v\b4\().4h
    saddw2  v\a1\().4s, v\a1\().4s, v\b4\().8h
    saddw   v\a2\().4s, v\a2\().4s, v\b5\().4h
    saddw2  v\a3\().4s, v\a3\().4s, v\b5\().8h
    saddw   v\a0\().4s, v\a0\().4s, v\b6\().4h
    saddw2  v\a1\().4s, v\a1\().4s, v\b6\().8h
    saddw   v\a2\().4s, v\a2\().4s, v\b7\().4h
    saddw2  v\a3\().4s, v\a3\().4s, v\b7\().8h

    // stage 2 sum
    add  v\a0\().4s, v\a0\().4s, v\a1\().4s
    add  v\a2\().4s, v\a2\().4s, v\a3\().4s

    // stage 3 sum
    add  v0.4s, v\a0\().4s, v\a2\().4s
    addv s0, v0.4s
.endm

.macro SATD_8X8 \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7

    HADAMARD_8X8_H \
    \a0, \a1, \a2, \a3, \a4, \a5, \a6, \a7, \
    \b0, \b1, \b2, \b3, \b4, \b5, \b6, \b7

    HADAMARD_8X8_V \
    \a0, \a1, \a2, \a3, \a4, \a5, \a6, \a7, \
    \b0, \b1, \b2, \b3, \b4, \b5, \b6, \b7

    SUM_HADAMARD_8X8 \
    \a0, \a1, \a2, \a3, \a4, \a5, \a6, \a7, \
    \b0, \b1, \b2, \b3, \b4, \b5, \b6, \b7
.endm


function satd8x8_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    #define subtotal    w9
    #define total       w10
    #define height      w13

    mov  height, 8
    mov  total, wzr

    //  0,  1;   2,  3
    //  4,  5;   6,  7
    // 16, 17;  20, 21
    // 18, 19;  22, 23

L(satd_w8):
    load_rows 0, 1, 2, src, dst, src_stride, dst_stride
    load_rows 4, 5, 6, src, dst, src_stride, dst_stride
    load_rows 16, 17, 20, src, dst, src_stride, dst_stride
    load_rows 18, 19, 22, src, dst, src_stride, dst_stride

    SATD_8X8 \
    0, 1, 4, 5, 16, 17, 18, 19, \
    2, 3, 6, 7, 20, 21, 22, 23

    fmov subtotal, s0
    add  total, subtotal, total

    subs height, height, #8
    bne  L(satd_w8)

    mov  w0, total
    normalize_8
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride

    #undef subtotal
    #undef total
    #undef height
endfunc

.macro DOUBLE_HADAMARD_8X8 \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7 \
       c0 c1 c2 c3 c4 c5 c6 c7

    // Horizontal transform

    butterfly v\b0, v\b1, v\a0, v\a1
    butterfly v\b2, v\b3, v\a2, v\a3
    butterfly v\b4, v\b5, v\a4, v\a5
    butterfly v\b6, v\b7, v\a6, v\a7
    butterfly v\a0, v\a1, v\c0, v\c1
    butterfly v\a2, v\a3, v\c2, v\c3
    butterfly v\a4, v\a5, v\c4, v\c5
    butterfly v\a6, v\a7, v\c6, v\c7

    interleave v\c0, v\c1, v\b0, v\b1
    interleave v\c2, v\c3, v\b2, v\b3
    interleave v\c4, v\c5, v\b4, v\b5
    interleave v\c6, v\c7, v\b6, v\b7
    interleave v\b0, v\b1, v\a0, v\a1
    interleave v\b2, v\b3, v\a2, v\a3
    interleave v\b4, v\b5, v\a4, v\a5
    interleave v\b6, v\b7, v\a6, v\a7

    butterfly v\a0, v\a2, v\c0, v\c2
    butterfly v\a1, v\a3, v\c1, v\c3
    butterfly v\a4, v\a6, v\c4, v\c6
    butterfly v\a5, v\a7, v\c5, v\c7
    butterfly v\c0, v\c2, v\b0, v\b2
    butterfly v\c1, v\c3, v\b1, v\b3
    butterfly v\c4, v\c6, v\b4, v\b6
    butterfly v\c5, v\c7, v\b5, v\b7

    interleave_pairs v\b0, v\b2, v\a0, v\a2
    interleave_pairs v\b1, v\b3, v\a1, v\a3
    interleave_pairs v\b4, v\b6, v\a4, v\a6
    interleave_pairs v\b5, v\b7, v\a5, v\a7
    interleave_pairs v\a0, v\a2, v\c0, v\c2
    interleave_pairs v\a1, v\a3, v\c1, v\c3
    interleave_pairs v\a4, v\a6, v\c4, v\c6
    interleave_pairs v\a5, v\a7, v\c5, v\c7

    butterfly v\c0, v\c4, v\b0, v\b4
    butterfly v\c1, v\c5, v\b1, v\b5
    butterfly v\c2, v\c6, v\b2, v\b6
    butterfly v\c3, v\c7, v\b3, v\b7
    butterfly v\b0, v\b4, v\a0, v\a4
    butterfly v\b1, v\b5, v\a1, v\a5
    butterfly v\b2, v\b6, v\a2, v\a6
    butterfly v\b3, v\b7, v\a3, v\a7

    interleave_quads v\a0, v\a4, v\c0, v\c4
    interleave_quads v\a1, v\a5, v\c1, v\c5
    interleave_quads v\a2, v\a6, v\c2, v\c6
    interleave_quads v\a3, v\a7, v\c3, v\c7
    interleave_quads v\c0, v\c4, v\b0, v\b4
    interleave_quads v\c1, v\c5, v\b1, v\b5
    interleave_quads v\c2, v\c6, v\b2, v\b6
    interleave_quads v\c3, v\c7, v\b3, v\b7

    // Vertical transform

    butterfly v\b0, v\b1, v\a0, v\a1
    butterfly v\b2, v\b3, v\a2, v\a3
    butterfly v\b4, v\b5, v\a4, v\a5
    butterfly v\b6, v\b7, v\a6, v\a7
    butterfly v\a0, v\a1, v\c0, v\c1
    butterfly v\a2, v\a3, v\c2, v\c3
    butterfly v\a4, v\a5, v\c4, v\c5
    butterfly v\a6, v\a7, v\c6, v\c7

    butterfly v\c0, v\c2, v\b0, v\b2
    butterfly v\c1, v\c3, v\b1, v\b3
    butterfly v\c4, v\c6, v\b4, v\b6
    butterfly v\c5, v\c7, v\b5, v\b7
    butterfly v\b0, v\b2, v\a0, v\a2
    butterfly v\b1, v\b3, v\a1, v\a3
    butterfly v\b4, v\b6, v\a4, v\a6
    butterfly v\b5, v\b7, v\a5, v\a7

    butterfly v\a0, v\a4, v\c0, v\c4
    butterfly v\a1, v\a5, v\c1, v\c5
    butterfly v\a2, v\a6, v\c2, v\c6
    butterfly v\a3, v\a7, v\c3, v\c7
    butterfly v\c0, v\c4, v\b0, v\b4
    butterfly v\c1, v\c5, v\b1, v\b5
    butterfly v\c2, v\c6, v\b2, v\b6
    butterfly v\c3, v\c7, v\b3, v\b7
.endm

.macro SUM_DOUBLE_HADAMARD_8X8 \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7 \
       c0 c1 c2 c3 c4 c5 c6 c7

    // absolute value of transform coefficients
    abs  v\a0\().8h, v\a0\().8h
    abs  v\a1\().8h, v\a1\().8h
    abs  v\a2\().8h, v\a2\().8h
    abs  v\a3\().8h, v\a3\().8h
    abs  v\a4\().8h, v\a4\().8h
    abs  v\a5\().8h, v\a5\().8h
    abs  v\a6\().8h, v\a6\().8h
    abs  v\a7\().8h, v\a7\().8h
    abs  v\c0\().8h, v\c0\().8h
    abs  v\c1\().8h, v\c1\().8h
    abs  v\c2\().8h, v\c2\().8h
    abs  v\c3\().8h, v\c3\().8h
    abs  v\c4\().8h, v\c4\().8h
    abs  v\c5\().8h, v\c5\().8h
    abs  v\c6\().8h, v\c6\().8h
    abs  v\c7\().8h, v\c7\().8h

    // stage 1 sum
    sxtl v\b0\().4s, v\a0\().4h
    sxtl v\b1\().4s, v\a1\().4h
    sxtl v\b2\().4s, v\a2\().4h
    sxtl v\b3\().4s, v\a3\().4h
    sxtl v\b4\().4s, v\a4\().4h
    sxtl v\b5\().4s, v\a5\().4h
    sxtl v\b6\().4s, v\a6\().4h
    sxtl v\b7\().4s, v\a7\().4h
    saddw2  v\b0\().4s, v\b0\().4s, v\a0\().8h
    saddw2  v\b1\().4s, v\b1\().4s, v\a1\().8h
    saddw2  v\b2\().4s, v\b2\().4s, v\a2\().8h
    saddw2  v\b3\().4s, v\b3\().4s, v\a3\().8h
    saddw2  v\b4\().4s, v\b4\().4s, v\a4\().8h
    saddw2  v\b5\().4s, v\b5\().4s, v\a5\().8h
    saddw2  v\b6\().4s, v\b6\().4s, v\a6\().8h
    saddw2  v\b7\().4s, v\b7\().4s, v\a7\().8h
    saddw   v\b0\().4s, v\b0\().4s, v\c0\().4h
    saddw2  v\b1\().4s, v\b1\().4s, v\c0\().8h
    saddw   v\b2\().4s, v\b2\().4s, v\c1\().4h
    saddw2  v\b3\().4s, v\b3\().4s, v\c1\().8h
    saddw   v\b4\().4s, v\b4\().4s, v\c2\().4h
    saddw2  v\b5\().4s, v\b5\().4s, v\c2\().8h
    saddw   v\b6\().4s, v\b6\().4s, v\c3\().4h
    saddw2  v\b7\().4s, v\b7\().4s, v\c3\().8h
    saddw   v\b0\().4s, v\b0\().4s, v\c4\().4h
    saddw2  v\b1\().4s, v\b1\().4s, v\c4\().8h
    saddw   v\b2\().4s, v\b2\().4s, v\c5\().4h
    saddw2  v\b3\().4s, v\b3\().4s, v\c5\().8h
    saddw   v\b4\().4s, v\b4\().4s, v\c6\().4h
    saddw2  v\b5\().4s, v\b5\().4s, v\c6\().8h
    saddw   v\b6\().4s, v\b6\().4s, v\c7\().4h
    saddw2  v\b7\().4s, v\b7\().4s, v\c7\().8h

    // stage 2 sum
    add  v\b0\().4s, v\b0\().4s, v\b1\().4s
    add  v\b2\().4s, v\b2\().4s, v\b3\().4s
    add  v\b4\().4s, v\b4\().4s, v\b5\().4s
    add  v\b6\().4s, v\b6\().4s, v\b7\().4s

    // stage 3 sum
    add  v\b0\().4s, v\b0\().4s, v\b2\().4s
    add  v\b4\().4s, v\b4\().4s, v\b6\().4s

    // stage 4 sum
    add  v0.4s, v\b0\().4s, v\b4\().4s
    addv s0, v0.4s
.endm

function satd16x8_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    #define subtotal    w9
    #define total       w10
    #define w_ext       x11
    #define w_bak       w11
    #define width       w12
    #define height      w13

    mov  height, 8
    mov  width, 16
    sxtw w_ext, width
    mov  total, wzr

    //  0,  1;   2,  3;  24, 25
    //  4,  5;   6,  7;  26, 27
    // 16, 17;  20, 21;  28, 29
    // 18, 19;  22, 23;  30, 31

L(satd_w16up):
    load_rows 0, 1, 2, src, dst, src_stride, dst_stride, 24, 25
    load_rows 4, 5, 6, src, dst, src_stride, dst_stride, 26, 27
    load_rows 16, 17, 20, src, dst, src_stride, dst_stride, 28, 29
    load_rows 18, 19, 22, src, dst, src_stride, dst_stride, 30, 31

    DOUBLE_HADAMARD_8X8 \
     0,  1,  4,  5, 16, 17, 18, 19, \
     2,  3,  6,  7, 20, 21, 22, 23, \
    24, 25, 26, 27, 28, 29, 30, 31

    SUM_DOUBLE_HADAMARD_8X8 \
     0,  1,  4,  5, 16, 17, 18, 19, \
     2,  3,  6,  7, 20, 21, 22, 23, \
    24, 25, 26, 27, 28, 29, 30, 31

    fmov subtotal, s0
    add  total, subtotal, total

    sub  src, src, src_stride, lsl 3
    sub  dst, dst, dst_stride, lsl 3
    add  src, src, #16
    add  dst, dst, #16
    subs width, width, #16
    bne  L(satd_w16up)

    sub  src, src, w_ext
    sub  dst, dst, w_ext
    add  src, src, src_stride, lsl 3
    add  dst, dst, dst_stride, lsl 3
    subs height, height, #8
    mov  width, w_bak
    bne  L(satd_w16up)

    mov  w0, total
    normalize_8
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride

    #undef w_ext
    #undef w_bak
    #undef subtotal
    #undef total
    #undef height
    #undef width
endfunc

.macro satd_x8up width, height
function satd\width\()x\height\()_neon, export=1
    mov  w13, \height
.if \width == 8
    mov  w10, wzr
    b    L(satd_w8)
.else
    mov  w12, \width
    sxtw x11, w12
    mov  w10, wzr
    b    L(satd_w16up)
.endif
endfunc
.endm

satd_x8up 8, 16
satd_x8up 8, 32
satd_x8up 16, 16
satd_x8up 16, 32
satd_x8up 16, 64
satd_x8up 32, 8
satd_x8up 32, 16
satd_x8up 32, 32
satd_x8up 32, 64
satd_x8up 64, 16
satd_x8up 64, 32
satd_x8up 64, 64
satd_x8up 64, 128
satd_x8up 128, 64
satd_x8up 128, 128

.macro load_rows_hbd n0, n1, n2, src, dst, src_stride, dst_stride
    ldr     q\n0, [\src]
    ldr     q\n1, [\dst]
    sub     v\n0\().8h, v\n0\().8h, v\n1\().8h

    ldr     q\n1, [\src, \src_stride]
    ldr     q\n2, [\dst, \dst_stride]
    sub     v\n1\().8h, v\n1\().8h, v\n2\().8h

    add     \src, \src, \src_stride, lsl 1
    add     \dst, \dst, \dst_stride, lsl 1
.endm

.macro HADAMARD_8X8_V_HBD \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7 \
       c0 c1 c2 c3 c4 c5 c6 c7

    // Vertical transform

    butterflyw v\b0, v\b1, v\c0, v\c1, v\a0, v\a1
    butterflyw v\b2, v\b3, v\c2, v\c3, v\a2, v\a3
    butterflyw v\b4, v\b5, v\c4, v\c5, v\a4, v\a5
    butterflyw v\b6, v\b7, v\c6, v\c7, v\a6, v\a7

    butterfly v\a0, v\a2, v\b0, v\b2, 4s
    butterfly v\a1, v\a3, v\b1, v\b3, 4s
    butterfly v\a4, v\a6, v\b4, v\b6, 4s
    butterfly v\a5, v\a7, v\b5, v\b7, 4s
    butterfly v\b0, v\b2, v\c0, v\c2, 4s
    butterfly v\b1, v\b3, v\c1, v\c3, 4s
    butterfly v\b4, v\b6, v\c4, v\c6, 4s
    butterfly v\b5, v\b7, v\c5, v\c7, 4s

    butterfly v\c0, v\c4, v\a0, v\a4, 4s
    butterfly v\c1, v\c5, v\a1, v\a5, 4s
    butterfly v\c2, v\c6, v\a2, v\a6, 4s
    butterfly v\c3, v\c7, v\a3, v\a7, 4s
    butterfly v\a0, v\a4, v\b0, v\b4, 4s
    butterfly v\a1, v\a5, v\b1, v\b5, 4s
    butterfly v\a2, v\a6, v\b2, v\b6, 4s
    butterfly v\a3, v\a7, v\b3, v\b7, 4s
.endm

.macro SUM_HADAMARD_8X8_HBD \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       c0 c1 c2 c3 c4 c5 c6 c7

    // absolute value of transform coefficients
    abs  v\a0\().4s, v\a0\().4s
    abs  v\a1\().4s, v\a1\().4s
    abs  v\a2\().4s, v\a2\().4s
    abs  v\a3\().4s, v\a3\().4s
    abs  v\a4\().4s, v\a4\().4s
    abs  v\a5\().4s, v\a5\().4s
    abs  v\a6\().4s, v\a6\().4s
    abs  v\a7\().4s, v\a7\().4s
    abs  v\c0\().4s, v\c0\().4s
    abs  v\c1\().4s, v\c1\().4s
    abs  v\c2\().4s, v\c2\().4s
    abs  v\c3\().4s, v\c3\().4s
    abs  v\c4\().4s, v\c4\().4s
    abs  v\c5\().4s, v\c5\().4s
    abs  v\c6\().4s, v\c6\().4s
    abs  v\c7\().4s, v\c7\().4s

    // stage 1 sum
    add  v\a0\().4s, v\a0\().4s, v\a1\().4s
    add  v\a2\().4s, v\a2\().4s, v\a3\().4s
    add  v\a4\().4s, v\a4\().4s, v\a5\().4s
    add  v\a6\().4s, v\a6\().4s, v\a7\().4s
    add  v\c0\().4s, v\c0\().4s, v\c1\().4s
    add  v\c2\().4s, v\c2\().4s, v\c3\().4s
    add  v\c4\().4s, v\c4\().4s, v\c5\().4s
    add  v\c6\().4s, v\c6\().4s, v\c7\().4s

    // stage 2 sum
    add  v\a0\().4s, v\a0\().4s, v\a2\().4s
    add  v\a4\().4s, v\a4\().4s, v\a6\().4s
    add  v\c0\().4s, v\c0\().4s, v\c2\().4s
    add  v\c4\().4s, v\c4\().4s, v\c6\().4s

    // stage 3 sum
    add  v\a0\().4s, v\a0\().4s, v\a4\().4s
    add  v\c0\().4s, v\c0\().4s, v\c4\().4s

    // stage 4 sum
    add  v0.4s, v\a0\().4s, v\c0\().4s
    addv s0, v0.4s
.endm


.macro SATD_8X8_HBD \
       a0 a1 a2 a3 a4 a5 a6 a7 \
       b0 b1 b2 b3 b4 b5 b6 b7 \
       c0 c1 c2 c3 c4 c5 c6 c7

    HADAMARD_8X8_H \
    \a0, \a1, \a2, \a3, \a4, \a5, \a6, \a7, \
    \b0, \b1, \b2, \b3, \b4, \b5, \b6, \b7

    HADAMARD_8X8_V_HBD \
    \a0, \a1, \a2, \a3, \a4, \a5, \a6, \a7, \
    \b0, \b1, \b2, \b3, \b4, \b5, \b6, \b7, \
    \c0, \c1, \c2, \c3, \c4, \c5, \c6, \c7

    SUM_HADAMARD_8X8_HBD \
    \a0, \a1, \a2, \a3, \a4, \a5, \a6, \a7, \
    \c0, \c1, \c2, \c3, \c4, \c5, \c6, \c7
.endm

function satd8x8_hbd_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    #define subtotal    w9
    #define total       w10
    #define w_ext       x11
    #define w_bak       w11
    #define width       w12
    #define height      w13

    mov  height, 8
    mov  width, 8
    sxtw w_ext, width
    mov  total, wzr

    //  0,  1;   2,  3;  24, 25
    //  4,  5;   6,  7;  26, 27
    // 16, 17;  20, 21;  28, 29
    // 18, 19;  22, 23;  30, 31

L(satd_w8up_hbd):
    load_rows_hbd 0, 1, 2, src, dst, src_stride, dst_stride
    load_rows_hbd 4, 5, 6, src, dst, src_stride, dst_stride
    load_rows_hbd 16, 17, 20, src, dst, src_stride, dst_stride
    load_rows_hbd 18, 19, 22, src, dst, src_stride, dst_stride

    SATD_8X8_HBD \
     0,  1,  4,  5, 16, 17, 18, 19, \
     2,  3,  6,  7, 20, 21, 22, 23, \
    24, 25, 26, 27, 28, 29, 30, 31

    fmov subtotal, s0
    add  total, subtotal, total

    sub  src, src, src_stride, lsl 3
    sub  dst, dst, dst_stride, lsl 3
    add  src, src, #16
    add  dst, dst, #16
    subs width, width, #8
    bne  L(satd_w8up_hbd)

    sub  src, src, w_ext, lsl 1
    sub  dst, dst, w_ext, lsl 1
    add  src, src, src_stride, lsl 3
    add  dst, dst, dst_stride, lsl 3
    subs height, height, #8
    mov  width, w_bak
    bne  L(satd_w8up_hbd)

    mov  w0, total
    normalize_8
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride

    #undef w_ext
    #undef w_bak
    #undef subtotal
    #undef total
    #undef height
    #undef width
endfunc

.macro satd_x8up_hbd width, height
function satd\width\()x\height\()_hbd_neon, export=1
    mov  w13, \height
    mov  w12, \width
    sxtw x11, w12
    mov  w10, wzr
    b    L(satd_w8up_hbd)
endfunc
.endm

satd_x8up_hbd 8, 16
satd_x8up_hbd 8, 32
satd_x8up_hbd 16, 8
satd_x8up_hbd 16, 16
satd_x8up_hbd 16, 32
satd_x8up_hbd 16, 64
satd_x8up_hbd 32, 8
satd_x8up_hbd 32, 16
satd_x8up_hbd 32, 32
satd_x8up_hbd 32, 64
satd_x8up_hbd 64, 16
satd_x8up_hbd 64, 32
satd_x8up_hbd 64, 64
satd_x8up_hbd 64, 128
satd_x8up_hbd 128, 64
satd_x8up_hbd 128, 128

// x0: src: *const u16,
// x1: src_stride: isize,
// x2: dst: *const u16,
// x3: dst_stride: isize,
function satd4x4_hbd_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    #define subtotal    w9
    #define total       w10
    #define height      w13

    mov  height, 4
    mov  total, wzr

L(satd_w4_hbd):
    ldr  d0, [src]
    ldr  d1, [dst]
    sub  v0.8h, v0.8h, v1.8h

    ldr  d1, [src, src_stride]
    ldr  d2, [dst, dst_stride]
    sub  v1.8h, v1.8h, v2.8h

    add  src, src, src_stride, lsl 1
    add  dst, dst, dst_stride, lsl 1

    ldr  d2, [src]
    ldr  d3, [dst]
    sub  v2.8h, v2.8h, v3.8h

    ldr  d3, [src, src_stride]
    ldr  d4, [dst, src_stride]
    sub  v3.8h, v3.8h, v4.8h

    add  src, src, src_stride, lsl 1
    add  dst, dst, dst_stride, lsl 1

    // pack rows 0-2, 1-3
    mov  v0.d[1], v2.d[0]
    mov  v1.d[1], v3.d[0]

    // Horizontal transform
    butterfly v2, v3, v0, v1
    interleave v0, v1, v2, v3
    butterfly v2, v3, v0, v1
    interleave_pairs v0, v1, v2, v3
    // Vertical transform

    butterfly v2, v3, v0, v1
    interleave v0, v1, v2, v3
    butterflyw v2, v3, v4, v5, v0, v1

    // absolute value of transform coefficients
    abs  v2.4s, v2.4s
    abs  v3.4s, v3.4s
    abs  v4.4s, v4.4s
    abs  v5.4s, v5.4s

    // stage 1 sum
    add  v2.4s, v2.4s, v3.4s
    add  v4.4s, v4.4s, v5.4s

    // stage 2 sum
    add  v0.4s, v2.4s, v4.4s
    addv s0, v0.4s

    fmov subtotal, s0
    add  total, subtotal, total

    subs height, height, #4
    bne  L(satd_w4_hbd)

    mov  w0, total
    normalize_4
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride

    #undef subtotal
    #undef total
    #undef height
endfunc

function satd4x8_hbd_neon, export=1
    mov  w13, 8
    mov  w10, wzr
    b    L(satd_w4_hbd)
endfunc

function satd4x16_hbd_neon, export=1
    mov  w13, 16
    mov  w10, wzr
    b    L(satd_w4_hbd)
endfunc

.macro SUM_DOUBLE_HADAMARD_4X4_HBD \
       a0 a1 a2 a3 c0 c1 c2 c3

    // absolute value of transform coefficients
    abs  v\a0\().4s, v\a0\().4s
    abs  v\a1\().4s, v\a1\().4s
    abs  v\a2\().4s, v\a2\().4s
    abs  v\a3\().4s, v\a3\().4s
    abs  v\c0\().4s, v\c0\().4s
    abs  v\c1\().4s, v\c1\().4s
    abs  v\c2\().4s, v\c2\().4s
    abs  v\c3\().4s, v\c3\().4s

    // stage 1 sum
    add  v\a0\().4s, v\a0\().4s, v\a1\().4s
    add  v\a2\().4s, v\a2\().4s, v\a3\().4s
    add  v\c0\().4s, v\c0\().4s, v\c1\().4s
    add  v\c2\().4s, v\c2\().4s, v\c3\().4s

    // stage 2 sum
    add  v\a0\().4s, v\a0\().4s, v\a2\().4s
    add  v\c0\().4s, v\c0\().4s, v\c2\().4s

    // stage 3 sum
    add  v0.4s, v\a0\().4s, v\c0\().4s
    addv s0, v0.4s
.endm

function satd8x4_hbd_neon, export=1
    #define src         x0
    #define src_stride  x1
    #define dst         x2
    #define dst_stride  x3

    #define subtotal    w9
    #define total       w10
    #define width       w12

    mov  width, 8
    mov  total, wzr

L(satd_h4_hbd):
    ldr  q0, [src]
    ldr  q1, [dst]
    sub  v0.8h, v0.8h, v1.8h

    ldr  q1, [src, src_stride]
    ldr  q2, [dst, dst_stride]
    sub  v1.8h, v1.8h, v2.8h

    lsl  x8, src_stride, 1
    lsl  x9, dst_stride, 1

    ldr  q2, [src, x8]
    ldr  q3, [dst, x9]
    sub  v2.8h, v2.8h, v3.8h

    add  x8, src_stride, src_stride, lsl 1
    add  x9, dst_stride, dst_stride, lsl 1

    ldr  q3, [src, x8]
    ldr  q4, [dst, x9]
    sub  v3.8h, v3.8h, v4.8h

    ext  v4.16b, v0.16b, v0.16b, 8
    ext  v5.16b, v1.16b, v1.16b, 8
    mov  v0.d[1], v2.d[0]
    mov  v1.d[1], v3.d[0]
    mov  v4.d[1], v2.d[1]
    mov  v5.d[1], v3.d[1]

    DOUBLE_HADAMARD_4X4 hbd=1
    SUM_DOUBLE_HADAMARD_4X4_HBD 2, 3, 16, 17, 6, 7, 18, 19

    fmov subtotal, s0
    add  total, subtotal, total

    add  src, src, #16
    add  dst, dst, #16
    subs width, width, #8
    bne  L(satd_h4_hbd)

    mov  w0, total
    normalize_4
    ret

    #undef src
    #undef src_stride
    #undef dst
    #undef dst_stride

    #undef subtotal
    #undef total
    #undef width
endfunc

function satd16x4_hbd_neon, export=1
    mov  w12, 16
    mov  w10, wzr
    b    L(satd_h4_hbd)
endfunc
