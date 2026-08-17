/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Martin Storsjo
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#include "src/arm/asm.S"
#include "util.S"

#define SUM_STRIDE (384+16)

// void dav1d_sgr_box3_v_neon(int32_t *sumsq, int16_t *sum,
//                            const int w, const int h,
//                            const enum LrEdgeFlags edges);
function sgr_box3_v_neon, export=1
        add             w10, w3,  #2 // Number of output rows to move back
        mov             w11, w3      // Number of input rows to move back
        add             w2,  w2,  #2 // Actual summed width
        mov             x7,       #(4*SUM_STRIDE) // sumsq stride
        mov             x8,       #(2*SUM_STRIDE) // sum stride
        sub             x0,  x0,  #(4*SUM_STRIDE) // sumsq -= stride
        sub             x1,  x1,  #(2*SUM_STRIDE) // sum   -= stride

        tst             w4,  #4 // LR_HAVE_TOP
        b.eq            0f
        // If have top, read from row -2.
        sub             x5,  x0,  #(4*SUM_STRIDE)
        sub             x6,  x1,  #(2*SUM_STRIDE)
        add             w11, w11, #2
        b               1f
0:
        // !LR_HAVE_TOP
        // If we don't have top, read from row 0 even if
        // we start writing to row -1.
        add             x5,  x0,  #(4*SUM_STRIDE)
        add             x6,  x1,  #(2*SUM_STRIDE)
1:

        tst             w4,  #8 // LR_HAVE_BOTTOM
        b.eq            1f
        // LR_HAVE_BOTTOM
        add             w3,  w3,  #2  // Sum all h+2 lines with the main loop
        add             w11, w11, #2
1:
        mov             w9,  w3       // Backup of h for next loops

1:
        // Start of horizontal loop; start one vertical filter slice.
        // Start loading rows into v16-v21 and v24-v26 taking top
        // padding into consideration.
        tst             w4,  #4 // LR_HAVE_TOP
        ld1             {v16.4s, v17.4s}, [x5], x7
        ld1             {v24.8h},         [x6], x8
        b.eq            2f
        // LR_HAVE_TOP
        ld1             {v18.4s, v19.4s}, [x5], x7
        ld1             {v25.8h},         [x6], x8
        ld1             {v20.4s, v21.4s}, [x5], x7
        ld1             {v26.8h},         [x6], x8
        b               3f
2:      // !LR_HAVE_TOP
        mov             v18.16b, v16.16b
        mov             v19.16b, v17.16b
        mov             v25.16b, v24.16b
        mov             v20.16b, v16.16b
        mov             v21.16b, v17.16b
        mov             v26.16b, v24.16b

3:
        subs            w3,  w3,  #1
.macro add3
        add             v16.4s,  v16.4s,  v18.4s
        add             v17.4s,  v17.4s,  v19.4s
        add             v24.8h,  v24.8h,  v25.8h
        add             v16.4s,  v16.4s,  v20.4s
        add             v17.4s,  v17.4s,  v21.4s
        add             v24.8h,  v24.8h,  v26.8h
        st1             {v16.4s, v17.4s}, [x0], x7
        st1             {v24.8h},         [x1], x8
.endm
        add3
        mov             v16.16b, v18.16b
        mov             v17.16b, v19.16b
        mov             v24.16b, v25.16b
        mov             v18.16b, v20.16b
        mov             v19.16b, v21.16b
        mov             v25.16b, v26.16b
        b.le            4f
        ld1             {v20.4s, v21.4s}, [x5], x7
        ld1             {v26.8h},         [x6], x8
        b               3b

4:
        tst             w4,  #8 // LR_HAVE_BOTTOM
        b.ne            5f
        // !LR_HAVE_BOTTOM
        // Produce two more rows, extending the already loaded rows.
        add3
        mov             v16.16b, v18.16b
        mov             v17.16b, v19.16b
        mov             v24.16b, v25.16b
        add3

5:      // End of one vertical slice.
        subs            w2,  w2,  #8
        b.le            0f
        // Move pointers back up to the top and loop horizontally.
        // Input pointers
        msub            x5,  x7,  x11, x5
        msub            x6,  x8,  x11, x6
        // Output pointers
        msub            x0,  x7,  x10, x0
        msub            x1,  x8,  x10, x1
        add             x0,  x0,  #32
        add             x1,  x1,  #16
        add             x5,  x5,  #32
        add             x6,  x6,  #16
        mov             w3,  w9
        b               1b

0:
        ret
.purgem add3
endfunc

// void dav1d_sgr_box5_v_neon(int32_t *sumsq, int16_t *sum,
//                            const int w, const int h,
//                            const enum LrEdgeFlags edges);
function sgr_box5_v_neon, export=1
        add             w10, w3,  #2 // Number of output rows to move back
        mov             w11, w3      // Number of input rows to move back
        add             w2,  w2,  #8 // Actual summed width
        mov             x7,       #(4*SUM_STRIDE) // sumsq stride
        mov             x8,       #(2*SUM_STRIDE) // sum stride
        sub             x0,  x0,  #(4*SUM_STRIDE) // sumsq -= stride
        sub             x1,  x1,  #(2*SUM_STRIDE) // sum   -= stride

        tst             w4,  #4 // LR_HAVE_TOP
        b.eq            0f
        // If have top, read from row -2.
        sub             x5,  x0,  #(4*SUM_STRIDE)
        sub             x6,  x1,  #(2*SUM_STRIDE)
        add             w11, w11, #2
        b               1f
0:
        // !LR_HAVE_TOP
        // If we don't have top, read from row 0 even if
        // we start writing to row -1.
        add             x5,  x0,  #(4*SUM_STRIDE)
        add             x6,  x1,  #(2*SUM_STRIDE)
1:

        tst             w4,  #8 // LR_HAVE_BOTTOM
        b.eq            0f
        // LR_HAVE_BOTTOM
        add             w3,  w3,  #2  // Handle h+2 lines with the main loop
        add             w11, w11, #2
        b               1f
0:
        // !LR_HAVE_BOTTOM
        sub             w3,  w3,  #1  // Handle h-1 lines with the main loop
1:
        mov             w9,  w3       // Backup of h for next loops

1:
        // Start of horizontal loop; start one vertical filter slice.
        // Start loading rows into v16-v25 and v26-v30 taking top
        // padding into consideration.
        tst             w4,  #4 // LR_HAVE_TOP
        ld1             {v16.4s, v17.4s}, [x5], x7
        ld1             {v26.8h},         [x6], x8
        b.eq            2f
        // LR_HAVE_TOP
        ld1             {v20.4s, v21.4s}, [x5], x7
        ld1             {v28.8h},         [x6], x8
        mov             v18.16b, v16.16b
        mov             v19.16b, v17.16b
        mov             v27.16b, v26.16b
        ld1             {v22.4s, v23.4s}, [x5], x7
        ld1             {v29.8h},         [x6], x8
        b               3f
2:      // !LR_HAVE_TOP
        mov             v18.16b, v16.16b
        mov             v19.16b, v17.16b
        mov             v27.16b, v26.16b
        mov             v20.16b, v16.16b
        mov             v21.16b, v17.16b
        mov             v28.16b, v26.16b
        mov             v22.16b, v16.16b
        mov             v23.16b, v17.16b
        mov             v29.16b, v26.16b

3:
        cbz             w3,  4f
        ld1             {v24.4s, v25.4s}, [x5], x7
        ld1             {v30.8h},         [x6], x8

3:
        // Start of vertical loop
        subs            w3,  w3,  #2
.macro add5
        add             v16.4s,  v16.4s,  v18.4s
        add             v17.4s,  v17.4s,  v19.4s
        add             v26.8h,  v26.8h,  v27.8h
        add             v0.4s,   v20.4s,  v22.4s
        add             v1.4s,   v21.4s,  v23.4s
        add             v2.8h,   v28.8h,  v29.8h
        add             v16.4s,  v16.4s,  v24.4s
        add             v17.4s,  v17.4s,  v25.4s
        add             v26.8h,  v26.8h,  v30.8h
        add             v16.4s,  v16.4s,  v0.4s
        add             v17.4s,  v17.4s,  v1.4s
        add             v26.8h,  v26.8h,  v2.8h
        st1             {v16.4s, v17.4s}, [x0], x7
        st1             {v26.8h},         [x1], x8
.endm
        add5
.macro shift2
        mov             v16.16b, v20.16b
        mov             v17.16b, v21.16b
        mov             v26.16b, v28.16b
        mov             v18.16b, v22.16b
        mov             v19.16b, v23.16b
        mov             v27.16b, v29.16b
        mov             v20.16b, v24.16b
        mov             v21.16b, v25.16b
        mov             v28.16b, v30.16b
.endm
        shift2
        add             x0,  x0,  x7
        add             x1,  x1,  x8
        b.le            5f
        ld1             {v22.4s, v23.4s}, [x5], x7
        ld1             {v29.8h},         [x6], x8
        ld1             {v24.4s, v25.4s}, [x5], x7
        ld1             {v30.8h},         [x6], x8
        b               3b

4:
        // h == 1, !LR_HAVE_BOTTOM.
        // Pad the last row with the only content row, and add.
        mov             v24.16b, v22.16b
        mov             v25.16b, v23.16b
        mov             v30.16b, v29.16b
        add5
        shift2
        add             x0,  x0,  x7
        add             x1,  x1,  x8
        add5
        b               6f

5:
        tst             w4,  #8 // LR_HAVE_BOTTOM
        b.ne            6f
        // !LR_HAVE_BOTTOM
        cbnz            w3,  5f
        // The intended three edge rows left; output the one at h-2 and
        // the past edge one at h.
        ld1             {v22.4s, v23.4s}, [x5], x7
        ld1             {v29.8h},         [x6], x8
        // Pad the past-edge row from the last content row.
        mov             v24.16b, v22.16b
        mov             v25.16b, v23.16b
        mov             v30.16b, v29.16b
        add5
        shift2
        add             x0,  x0,  x7
        add             x1,  x1,  x8
        // The last two rows are already padded properly here.
        add5
        b               6f

5:
        // w3 == -1, two rows left, output one.
        // Pad the last two rows from the mid one.
        mov             v22.16b, v20.16b
        mov             v23.16b, v21.16b
        mov             v29.16b, v28.16b
        mov             v24.16b, v20.16b
        mov             v25.16b, v21.16b
        mov             v30.16b, v28.16b
        add5
        add             x0,  x0,  x7
        add             x1,  x1,  x8
        b               6f

6:      // End of one vertical slice.
        subs            w2,  w2,  #8
        b.le            0f
        // Move pointers back up to the top and loop horizontally.
        // Input pointers
        msub            x5,  x7,  x11, x5
        msub            x6,  x8,  x11, x6
        // Output pointers
        msub            x0,  x7,  x10, x0
        msub            x1,  x8,  x10, x1
        add             x0,  x0,  #32
        add             x1,  x1,  #16
        add             x5,  x5,  #32
        add             x6,  x6,  #16
        mov             w3,  w9
        b               1b

0:
        ret
.purgem add5
endfunc

// void dav1d_sgr_calc_ab1_neon(int32_t *a, int16_t *b,
//                              const int w, const int h, const int strength,
//                              const int bitdepth_max);
// void dav1d_sgr_calc_ab2_neon(int32_t *a, int16_t *b,
//                              const int w, const int h, const int strength,
//                              const int bitdepth_max);
function sgr_calc_ab1_neon, export=1
        clz             w9,  w5
        add             x3,  x3,  #2 // h += 2
        movi            v31.4s,   #9 // n
        mov             x5,  #455
        mov             x8,  #SUM_STRIDE
        b               sgr_calc_ab_neon
endfunc

function sgr_calc_ab2_neon, export=1
        clz             w9,  w5
        add             x3,  x3,  #3  // h += 3
        asr             x3,  x3,  #1  // h /= 2
        movi            v31.4s,   #25 // n
        mov             x5,  #164
        mov             x8,  #(2*SUM_STRIDE)
endfunc

function sgr_calc_ab_neon
        sub             w9,  w9,  #24  // -bitdepth_min_8
        movrel          x12, X(sgr_x_by_x)
        ld1             {v16.16b, v17.16b, v18.16b}, [x12]
        dup             v6.8h,    w9   // -bitdepth_min_8
        movi            v19.16b,  #5
        movi            v20.8b,   #55  // idx of last 5
        movi            v21.8b,   #72  // idx of last 4
        movi            v22.8b,   #101 // idx of last 3
        movi            v23.8b,   #169 // idx of last 2
        movi            v24.8b,   #254 // idx of last 1
        saddl           v7.4s,    v6.4h,   v6.4h  // -2*bitdepth_min_8
        add             x2,  x2,  #2 // w += 2
        add             x7,  x2,  #7
        bic             x7,  x7,  #7 // aligned w
        sub             x7,  x8,  x7 // increment between rows
        movi            v29.8h,   #1, lsl #8
        dup             v28.4s,   w4
        dup             v30.4s,   w5 // one_by_x
        sub             x0,  x0,  #(4*(SUM_STRIDE))
        sub             x1,  x1,  #(2*(SUM_STRIDE))
        mov             x6,  x2   // backup of w
        sub             v16.16b, v16.16b, v19.16b
        sub             v17.16b, v17.16b, v19.16b
        sub             v18.16b, v18.16b, v19.16b
1:
        subs            x2,  x2,  #8
        ld1             {v0.4s, v1.4s}, [x0]   // a
        ld1             {v2.8h}, [x1]          // b
        srshl           v0.4s,  v0.4s,  v7.4s
        srshl           v1.4s,  v1.4s,  v7.4s
        srshl           v4.8h,  v2.8h,  v6.8h
        mul             v0.4s,  v0.4s,  v31.4s // a * n
        mul             v1.4s,  v1.4s,  v31.4s // a * n
        umull           v3.4s,  v4.4h,  v4.4h  // b * b
        umull2          v4.4s,  v4.8h,  v4.8h  // b * b
        uqsub           v0.4s,  v0.4s,  v3.4s  // imax(a * n - b * b, 0)
        uqsub           v1.4s,  v1.4s,  v4.4s  // imax(a * n - b * b, 0)
        mul             v0.4s,  v0.4s,  v28.4s // p * s
        mul             v1.4s,  v1.4s,  v28.4s // p * s
        uqshrn          v0.4h,  v0.4s,  #16
        uqshrn2         v0.8h,  v1.4s,  #16
        uqrshrn         v0.8b,  v0.8h,  #4     // imin(z, 255)

        cmhi            v25.8b, v0.8b,  v20.8b // = -1 if sgr_x_by_x[v0] < 5
        cmhi            v26.8b, v0.8b,  v21.8b // = -1 if sgr_x_by_x[v0] < 4
        tbl             v1.8b, {v16.16b, v17.16b, v18.16b}, v0.8b
        cmhi            v27.8b, v0.8b,  v22.8b // = -1 if sgr_x_by_x[v0] < 3
        cmhi            v4.8b,  v0.8b,  v23.8b // = -1 if sgr_x_by_x[v0] < 2
        add             v25.8b, v25.8b, v26.8b
        cmhi            v5.8b,  v0.8b,  v24.8b // = -1 if sgr_x_by_x[v0] < 1
        add             v27.8b, v27.8b, v4.8b
        add             v5.8b,  v5.8b,  v19.8b
        add             v25.8b, v25.8b, v27.8b
        add             v1.8b,  v1.8b,  v5.8b
        add             v1.8b,  v1.8b,  v25.8b
        uxtl            v1.8h,  v1.8b          // x

        umull           v3.4s,  v1.4h,  v2.4h  // x * BB[i]
        umull2          v4.4s,  v1.8h,  v2.8h  // x * BB[i]
        mul             v3.4s,  v3.4s,  v30.4s // x * BB[i] * sgr_one_by_x
        mul             v4.4s,  v4.4s,  v30.4s // x * BB[i] * sgr_one_by_x
        srshr           v3.4s,  v3.4s,  #12    // AA[i]
        srshr           v4.4s,  v4.4s,  #12    // AA[i]
        sub             v2.8h,  v29.8h, v1.8h  // 256 - x

        st1             {v3.4s, v4.4s}, [x0], #32
        st1             {v2.8h}, [x1], #16
        b.gt            1b

        subs            x3,  x3,  #1
        b.le            0f
        add             x0,  x0,  x7, lsl #2
        add             x1,  x1,  x7, lsl #1
        mov             x2,  x6
        b               1b
0:
        ret
endfunc
