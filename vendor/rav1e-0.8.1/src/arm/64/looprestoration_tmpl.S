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

#define FILTER_OUT_STRIDE 384

.macro sgr_funcs bpc
// void dav1d_sgr_finish_filter1_Xbpc_neon(int16_t *tmp,
//                                         const pixel *src, const ptrdiff_t stride,
//                                         const int32_t *a, const int16_t *b,
//                                         const int w, const int h);
function sgr_finish_filter1_\bpc\()bpc_neon, export=1
        sub             x7,  x3,  #(4*SUM_STRIDE)
        add             x8,  x3,  #(4*SUM_STRIDE)
        sub             x9,  x4,  #(2*SUM_STRIDE)
        add             x10, x4,  #(2*SUM_STRIDE)
        mov             x11, #SUM_STRIDE
        mov             x12, #FILTER_OUT_STRIDE
        add             x13, x5,  #7
        bic             x13, x13, #7 // Aligned width
.if \bpc == 8
        sub             x2,  x2,  x13
.else
        sub             x2,  x2,  x13, lsl #1
.endif
        sub             x12, x12, x13
        sub             x11, x11, x13
        sub             x11, x11, #4 // We read 4 extra elements from a
        sub             x14, x11, #4 // We read 8 extra elements from b
        mov             x13, x5
        movi            v6.8h,  #3
        movi            v7.4s,  #3
1:
        ld1             {v0.8h, v1.8h}, [x9], #32
        ld1             {v2.8h, v3.8h}, [x4], #32
        ld1             {v4.8h, v5.8h}, [x10], #32
        ld1             {v16.4s, v17.4s, v18.4s}, [x7], #48
        ld1             {v19.4s, v20.4s, v21.4s}, [x3], #48
        ld1             {v22.4s, v23.4s, v24.4s}, [x8], #48

2:
        subs            x5,  x5,  #8
        ext             v25.16b, v0.16b,  v1.16b, #2  // -stride
        ext             v26.16b, v2.16b,  v3.16b, #2  // 0
        ext             v27.16b, v4.16b,  v5.16b, #2  // +stride
        ext             v28.16b, v0.16b,  v1.16b, #4  // +1-stride
        ext             v29.16b, v2.16b,  v3.16b, #4  // +1
        ext             v30.16b, v4.16b,  v5.16b, #4  // +1+stride
        add             v2.8h,   v2.8h,   v25.8h      // -1, -stride
        add             v26.8h,  v26.8h,  v27.8h      // 0, +stride
        add             v0.8h,   v0.8h,   v28.8h      // -1-stride, +1-stride
        add             v2.8h,   v2.8h,   v26.8h
        add             v4.8h,   v4.8h,   v30.8h      // -1+stride, +1+stride
        add             v2.8h,   v2.8h,   v29.8h      // +1
        add             v0.8h,   v0.8h,   v4.8h

        ext             v25.16b, v16.16b, v17.16b, #4 // -stride
        ext             v26.16b, v17.16b, v18.16b, #4
        shl             v2.8h,   v2.8h,   #2
        ext             v27.16b, v16.16b, v17.16b, #8 // +1-stride
        ext             v28.16b, v17.16b, v18.16b, #8
        ext             v29.16b, v19.16b, v20.16b, #4 // 0
        ext             v30.16b, v20.16b, v21.16b, #4
        mla             v2.8h,   v0.8h,   v6.8h       // * 3 -> a
        add             v25.4s,  v25.4s,  v19.4s      // -stride, -1
        add             v26.4s,  v26.4s,  v20.4s
        add             v16.4s,  v16.4s,  v27.4s      // -1-stride, +1-stride
        add             v17.4s,  v17.4s,  v28.4s
        ext             v27.16b, v19.16b, v20.16b, #8 // +1
        ext             v28.16b, v20.16b, v21.16b, #8
        add             v16.4s,  v16.4s,  v22.4s      // -1+stride
        add             v17.4s,  v17.4s,  v23.4s
        add             v29.4s,  v29.4s,  v27.4s      // 0, +1
        add             v30.4s,  v30.4s,  v28.4s
        add             v25.4s,  v25.4s,  v29.4s
        add             v26.4s,  v26.4s,  v30.4s
        ext             v27.16b, v22.16b, v23.16b, #4 // +stride
        ext             v28.16b, v23.16b, v24.16b, #4
        ext             v29.16b, v22.16b, v23.16b, #8 // +1+stride
        ext             v30.16b, v23.16b, v24.16b, #8
.if \bpc == 8
        ld1             {v19.8b}, [x1], #8            // src
.else
        ld1             {v19.8h}, [x1], #16           // src
.endif
        add             v25.4s,  v25.4s,  v27.4s      // +stride
        add             v26.4s,  v26.4s,  v28.4s
        add             v16.4s,  v16.4s,  v29.4s      // +1+stride
        add             v17.4s,  v17.4s,  v30.4s
        shl             v25.4s,  v25.4s,  #2
        shl             v26.4s,  v26.4s,  #2
        mla             v25.4s,  v16.4s,  v7.4s       // * 3 -> b
        mla             v26.4s,  v17.4s,  v7.4s
.if \bpc == 8
        uxtl            v19.8h,  v19.8b               // src
.endif
        mov             v0.16b,  v1.16b
        umlal           v25.4s,  v2.4h,   v19.4h      // b + a * src
        umlal2          v26.4s,  v2.8h,   v19.8h
        mov             v2.16b,  v3.16b
        rshrn           v25.4h,  v25.4s,  #9
        rshrn2          v25.8h,  v26.4s,  #9
        mov             v4.16b,  v5.16b
        st1             {v25.8h}, [x0], #16

        b.le            3f
        mov             v16.16b, v18.16b
        mov             v19.16b, v21.16b
        mov             v22.16b, v24.16b
        ld1             {v1.8h}, [x9], #16
        ld1             {v3.8h}, [x4], #16
        ld1             {v5.8h}, [x10], #16
        ld1             {v17.4s, v18.4s}, [x7], #32
        ld1             {v20.4s, v21.4s}, [x3], #32
        ld1             {v23.4s, v24.4s}, [x8], #32
        b               2b

3:
        subs            x6,  x6,  #1
        b.le            0f
        mov             x5,  x13
        add             x0,  x0,  x12, lsl #1
        add             x1,  x1,  x2
        add             x3,  x3,  x11, lsl #2
        add             x7,  x7,  x11, lsl #2
        add             x8,  x8,  x11, lsl #2
        add             x4,  x4,  x14, lsl #1
        add             x9,  x9,  x14, lsl #1
        add             x10, x10, x14, lsl #1
        b               1b
0:
        ret
endfunc

// void dav1d_sgr_finish_filter2_Xbpc_neon(int16_t *tmp,
//                                         const pixel *src, const ptrdiff_t stride,
//                                         const int32_t *a, const int16_t *b,
//                                         const int w, const int h);
function sgr_finish_filter2_\bpc\()bpc_neon, export=1
        add             x7,  x3,  #(4*(SUM_STRIDE))
        sub             x3,  x3,  #(4*(SUM_STRIDE))
        add             x8,  x4,  #(2*(SUM_STRIDE))
        sub             x4,  x4,  #(2*(SUM_STRIDE))
        mov             x9,  #(2*SUM_STRIDE)
        mov             x10, #FILTER_OUT_STRIDE
        add             x11, x5,  #7
        bic             x11, x11, #7 // Aligned width
.if \bpc == 8
        sub             x2,  x2,  x11
.else
        sub             x2,  x2,  x11, lsl #1
.endif
        sub             x10, x10, x11
        sub             x9,  x9,  x11
        sub             x9,  x9,  #4 // We read 4 extra elements from a
        sub             x12, x9,  #4 // We read 8 extra elements from b
        mov             x11, x5
        movi            v4.8h,  #5
        movi            v5.4s,  #5
        movi            v6.8h,  #6
        movi            v7.4s,  #6
1:
        ld1             {v0.8h, v1.8h}, [x4], #32
        ld1             {v2.8h, v3.8h}, [x8], #32
        ld1             {v16.4s, v17.4s, v18.4s}, [x3], #48
        ld1             {v19.4s, v20.4s, v21.4s}, [x7], #48

2:
        subs            x5,  x5,  #8
        ext             v24.16b, v0.16b,  v1.16b, #4  // +1-stride
        ext             v25.16b, v2.16b,  v3.16b, #4  // +1+stride
        ext             v22.16b, v0.16b,  v1.16b, #2  // -stride
        ext             v23.16b, v2.16b,  v3.16b, #2  // +stride
        add             v0.8h,   v0.8h,   v24.8h      // -1-stride, +1-stride
        add             v25.8h,  v2.8h,   v25.8h      // -1+stride, +1+stride
        add             v2.8h,   v22.8h,  v23.8h      // -stride, +stride
        add             v0.8h,   v0.8h,   v25.8h

        ext             v22.16b, v16.16b, v17.16b, #4 // -stride
        ext             v23.16b, v17.16b, v18.16b, #4
        ext             v24.16b, v19.16b, v20.16b, #4 // +stride
        ext             v25.16b, v20.16b, v21.16b, #4
        ext             v26.16b, v16.16b, v17.16b, #8 // +1-stride
        ext             v27.16b, v17.16b, v18.16b, #8
        ext             v28.16b, v19.16b, v20.16b, #8 // +1+stride
        ext             v29.16b, v20.16b, v21.16b, #8
        mul             v0.8h,   v0.8h,   v4.8h       // * 5
        mla             v0.8h,   v2.8h,   v6.8h       // * 6
.if \bpc == 8
        ld1             {v31.8b}, [x1], #8
.else
        ld1             {v31.8h}, [x1], #16
.endif
        add             v16.4s,  v16.4s,  v26.4s      // -1-stride, +1-stride
        add             v17.4s,  v17.4s,  v27.4s
        add             v19.4s,  v19.4s,  v28.4s      // -1+stride, +1+stride
        add             v20.4s,  v20.4s,  v29.4s
        add             v16.4s,  v16.4s,  v19.4s
        add             v17.4s,  v17.4s,  v20.4s

        add             v22.4s,  v22.4s,  v24.4s      // -stride, +stride
        add             v23.4s,  v23.4s,  v25.4s
        // This is, surprisingly, faster than other variants where the
        // mul+mla pairs are further apart, on Cortex A53.
        mul             v16.4s,  v16.4s,  v5.4s       // * 5
        mla             v16.4s,  v22.4s,  v7.4s       // * 6
        mul             v17.4s,  v17.4s,  v5.4s       // * 5
        mla             v17.4s,  v23.4s,  v7.4s       // * 6

.if \bpc == 8
        uxtl            v31.8h,  v31.8b
.endif
        umlal           v16.4s,  v0.4h,   v31.4h      // b + a * src
        umlal2          v17.4s,  v0.8h,   v31.8h
        mov             v0.16b,  v1.16b
        rshrn           v16.4h,  v16.4s,  #9
        rshrn2          v16.8h,  v17.4s,  #9
        mov             v2.16b,  v3.16b
        st1             {v16.8h}, [x0], #16

        b.le            3f
        mov             v16.16b, v18.16b
        mov             v19.16b, v21.16b
        ld1             {v1.8h}, [x4], #16
        ld1             {v3.8h}, [x8], #16
        ld1             {v17.4s, v18.4s}, [x3], #32
        ld1             {v20.4s, v21.4s}, [x7], #32
        b               2b

3:
        subs            x6,  x6,  #1
        b.le            0f
        mov             x5,  x11
        add             x0,  x0,  x10, lsl #1
        add             x1,  x1,  x2
        add             x3,  x3,  x9, lsl #2
        add             x7,  x7,  x9, lsl #2
        add             x4,  x4,  x12, lsl #1
        add             x8,  x8,  x12, lsl #1
        mov             x13, x3
        mov             x14, x4

        ld1             {v0.8h, v1.8h}, [x4], #32
        ld1             {v16.4s, v17.4s, v18.4s}, [x3], #48

4:
        subs            x5,  x5,  #8
        ext             v23.16b, v0.16b,  v1.16b, #4  // +1
        ext             v22.16b, v0.16b,  v1.16b, #2  // 0
        add             v0.8h,   v0.8h,   v23.8h      // -1, +1

        ext             v24.16b, v16.16b, v17.16b, #4 // 0
        ext             v25.16b, v17.16b, v18.16b, #4
        ext             v26.16b, v16.16b, v17.16b, #8 // +1
        ext             v27.16b, v17.16b, v18.16b, #8
        mul             v2.8h,   v22.8h,  v6.8h       // * 6
        mla             v2.8h,   v0.8h,   v4.8h       // * 5 -> a
.if \bpc == 8
        ld1             {v31.8b}, [x1], #8
.else
        ld1             {v31.8h}, [x1], #16
.endif
        add             v16.4s,  v16.4s,  v26.4s      // -1, +1
        add             v17.4s,  v17.4s,  v27.4s
.if \bpc == 8
        uxtl            v31.8h,  v31.8b
.endif
        // This is, surprisingly, faster than other variants where the
        // mul+mla pairs are further apart, on Cortex A53.
        mul             v24.4s,  v24.4s,  v7.4s       // * 6
        mla             v24.4s,  v16.4s,  v5.4s       // * 5 -> b
        mul             v25.4s,  v25.4s,  v7.4s       // * 6
        mla             v25.4s,  v17.4s,  v5.4s       // * 5 -> b

        umlal           v24.4s,  v2.4h,   v31.4h      // b + a * src
        umlal2          v25.4s,  v2.8h,   v31.8h
        mov             v0.16b,  v1.16b
        rshrn           v24.4h,  v24.4s,  #8
        rshrn2          v24.8h,  v25.4s,  #8
        mov             v16.16b, v18.16b
        st1             {v24.8h}, [x0], #16

        b.le            5f
        ld1             {v1.8h}, [x4], #16
        ld1             {v17.4s, v18.4s}, [x3], #32
        b               4b

5:
        subs            x6,  x6,  #1
        b.le            0f
        mov             x5,  x11
        add             x0,  x0,  x10, lsl #1
        add             x1,  x1,  x2
        mov             x3,  x13 // Rewind x3/x4 to where they started
        mov             x4,  x14
        b               1b
0:
        ret
endfunc

// void dav1d_sgr_weighted1_Xbpc_neon(pixel *dst, const ptrdiff_t dst_stride,
//                                    const pixel *src, const ptrdiff_t src_stride,
//                                    const int16_t *t1, const int w, const int h,
//                                    const int wt, const int bitdepth_max);
function sgr_weighted1_\bpc\()bpc_neon, export=1
.if \bpc == 16
        ldr             w8,  [sp]
.endif
        dup             v31.8h, w7
        cmp             x6,  #2
.if \bpc == 16
        dup             v30.8h, w8
.endif
        add             x9,  x0,  x1
        add             x10, x2,  x3
        add             x11, x4,  #2*FILTER_OUT_STRIDE
        mov             x7,  #(4*FILTER_OUT_STRIDE)
        lsl             x1,  x1,  #1
        lsl             x3,  x3,  #1
        add             x8,  x5,  #7
        bic             x8,  x8,  #7 // Aligned width
.if \bpc == 8
        sub             x1,  x1,  x8
        sub             x3,  x3,  x8
.else
        sub             x1,  x1,  x8, lsl #1
        sub             x3,  x3,  x8, lsl #1
.endif
        sub             x7,  x7,  x8, lsl #1
        mov             x8,  x5
        b.lt            2f
1:
.if \bpc == 8
        ld1             {v0.8b}, [x2],  #8
        ld1             {v4.8b}, [x10], #8
.else
        ld1             {v0.8h}, [x2],  #16
        ld1             {v4.8h}, [x10], #16
.endif
        ld1             {v1.8h}, [x4],  #16
        ld1             {v5.8h}, [x11], #16
        subs            x5,  x5,  #8
.if \bpc == 8
        ushll           v0.8h,  v0.8b,  #4     // u
        ushll           v4.8h,  v4.8b,  #4     // u
.else
        shl             v0.8h,  v0.8h,  #4     // u
        shl             v4.8h,  v4.8h,  #4     // u
.endif
        sub             v1.8h,  v1.8h,  v0.8h  // t1 - u
        sub             v5.8h,  v5.8h,  v4.8h  // t1 - u
        ushll           v2.4s,  v0.4h,  #7     // u << 7
        ushll2          v3.4s,  v0.8h,  #7     // u << 7
        ushll           v6.4s,  v4.4h,  #7     // u << 7
        ushll2          v7.4s,  v4.8h,  #7     // u << 7
        smlal           v2.4s,  v1.4h,  v31.4h // v
        smlal2          v3.4s,  v1.8h,  v31.8h // v
        smlal           v6.4s,  v5.4h,  v31.4h // v
        smlal2          v7.4s,  v5.8h,  v31.8h // v
.if \bpc == 8
        rshrn           v2.4h,  v2.4s,  #11
        rshrn2          v2.8h,  v3.4s,  #11
        rshrn           v6.4h,  v6.4s,  #11
        rshrn2          v6.8h,  v7.4s,  #11
        sqxtun          v2.8b,  v2.8h
        sqxtun          v6.8b,  v6.8h
        st1             {v2.8b}, [x0], #8
        st1             {v6.8b}, [x9], #8
.else
        sqrshrun        v2.4h,  v2.4s,  #11
        sqrshrun2       v2.8h,  v3.4s,  #11
        sqrshrun        v6.4h,  v6.4s,  #11
        sqrshrun2       v6.8h,  v7.4s,  #11
        umin            v2.8h,  v2.8h,  v30.8h
        umin            v6.8h,  v6.8h,  v30.8h
        st1             {v2.8h}, [x0], #16
        st1             {v6.8h}, [x9], #16
.endif
        b.gt            1b

        sub             x6,  x6,  #2
        cmp             x6,  #1
        b.lt            0f
        mov             x5,  x8
        add             x0,  x0,  x1
        add             x9,  x9,  x1
        add             x2,  x2,  x3
        add             x10, x10, x3
        add             x4,  x4,  x7
        add             x11, x11, x7
        b.eq            2f
        b               1b

2:
.if \bpc == 8
        ld1             {v0.8b}, [x2], #8
.else
        ld1             {v0.8h}, [x2], #16
.endif
        ld1             {v1.8h}, [x4], #16
        subs            x5,  x5,  #8
.if \bpc == 8
        ushll           v0.8h,  v0.8b,  #4     // u
.else
        shl             v0.8h,  v0.8h,  #4     // u
.endif
        sub             v1.8h,  v1.8h,  v0.8h  // t1 - u
        ushll           v2.4s,  v0.4h,  #7     // u << 7
        ushll2          v3.4s,  v0.8h,  #7     // u << 7
        smlal           v2.4s,  v1.4h,  v31.4h // v
        smlal2          v3.4s,  v1.8h,  v31.8h // v
.if \bpc == 8
        rshrn           v2.4h,  v2.4s,  #11
        rshrn2          v2.8h,  v3.4s,  #11
        sqxtun          v2.8b,  v2.8h
        st1             {v2.8b}, [x0], #8
.else
        sqrshrun        v2.4h,  v2.4s,  #11
        sqrshrun2       v2.8h,  v3.4s,  #11
        umin            v2.8h,  v2.8h,  v30.8h
        st1             {v2.8h}, [x0], #16
.endif
        b.gt            2b
0:
        ret
endfunc

// void dav1d_sgr_weighted2_Xbpc_neon(pixel *dst, const ptrdiff_t stride,
//                                    const pixel *src, const ptrdiff_t src_stride,
//                                    const int16_t *t1, const int16_t *t2,
//                                    const int w, const int h,
//                                    const int16_t wt[2], const int bitdepth_max);
function sgr_weighted2_\bpc\()bpc_neon, export=1
.if \bpc == 8
        ldr             x8,  [sp]
.else
        ldp             x8,  x9,  [sp]
.endif
        cmp             x7,  #2
        add             x10, x0,  x1
        add             x11, x2,  x3
        add             x12, x4,  #2*FILTER_OUT_STRIDE
        add             x13, x5,  #2*FILTER_OUT_STRIDE
        ld2r            {v30.8h, v31.8h}, [x8] // wt[0], wt[1]
.if \bpc == 16
        dup             v29.8h,  w9
.endif
        mov             x8,  #4*FILTER_OUT_STRIDE
        lsl             x1,  x1,  #1
        lsl             x3,  x3,  #1
        add             x9,  x6,  #7
        bic             x9,  x9,  #7 // Aligned width
.if \bpc == 8
        sub             x1,  x1,  x9
        sub             x3,  x3,  x9
.else
        sub             x1,  x1,  x9, lsl #1
        sub             x3,  x3,  x9, lsl #1
.endif
        sub             x8,  x8,  x9, lsl #1
        mov             x9,  x6
        b.lt            2f
1:
.if \bpc == 8
        ld1             {v0.8b},  [x2],  #8
        ld1             {v16.8b}, [x11], #8
.else
        ld1             {v0.8h},  [x2],  #16
        ld1             {v16.8h}, [x11], #16
.endif
        ld1             {v1.8h},  [x4],  #16
        ld1             {v17.8h}, [x12], #16
        ld1             {v2.8h},  [x5],  #16
        ld1             {v18.8h}, [x13], #16
        subs            x6,  x6,  #8
.if \bpc == 8
        ushll           v0.8h,  v0.8b,  #4     // u
        ushll           v16.8h, v16.8b, #4     // u
.else
        shl             v0.8h,  v0.8h,  #4     // u
        shl             v16.8h, v16.8h, #4     // u
.endif
        sub             v1.8h,  v1.8h,  v0.8h  // t1 - u
        sub             v2.8h,  v2.8h,  v0.8h  // t2 - u
        sub             v17.8h, v17.8h, v16.8h // t1 - u
        sub             v18.8h, v18.8h, v16.8h // t2 - u
        ushll           v3.4s,  v0.4h,  #7     // u << 7
        ushll2          v4.4s,  v0.8h,  #7     // u << 7
        ushll           v19.4s, v16.4h, #7     // u << 7
        ushll2          v20.4s, v16.8h, #7     // u << 7
        smlal           v3.4s,  v1.4h,  v30.4h // wt[0] * (t1 - u)
        smlal           v3.4s,  v2.4h,  v31.4h // wt[1] * (t2 - u)
        smlal2          v4.4s,  v1.8h,  v30.8h // wt[0] * (t1 - u)
        smlal2          v4.4s,  v2.8h,  v31.8h // wt[1] * (t2 - u)
        smlal           v19.4s, v17.4h, v30.4h // wt[0] * (t1 - u)
        smlal           v19.4s, v18.4h, v31.4h // wt[1] * (t2 - u)
        smlal2          v20.4s, v17.8h, v30.8h // wt[0] * (t1 - u)
        smlal2          v20.4s, v18.8h, v31.8h // wt[1] * (t2 - u)
.if \bpc == 8
        rshrn           v3.4h,  v3.4s,  #11
        rshrn2          v3.8h,  v4.4s,  #11
        rshrn           v19.4h, v19.4s, #11
        rshrn2          v19.8h, v20.4s, #11
        sqxtun          v3.8b,  v3.8h
        sqxtun          v19.8b, v19.8h
        st1             {v3.8b},  [x0],  #8
        st1             {v19.8b}, [x10], #8
.else
        sqrshrun        v3.4h,  v3.4s,  #11
        sqrshrun2       v3.8h,  v4.4s,  #11
        sqrshrun        v19.4h, v19.4s, #11
        sqrshrun2       v19.8h, v20.4s, #11
        umin            v3.8h,  v3.8h,  v29.8h
        umin            v19.8h, v19.8h, v29.8h
        st1             {v3.8h},  [x0],  #16
        st1             {v19.8h}, [x10], #16
.endif
        b.gt            1b

        subs            x7,  x7,  #2
        cmp             x7,  #1
        b.lt            0f
        mov             x6,  x9
        add             x0,  x0,  x1
        add             x10, x10, x1
        add             x2,  x2,  x3
        add             x11, x11, x3
        add             x4,  x4,  x8
        add             x12, x12, x8
        add             x5,  x5,  x8
        add             x13, x13, x8
        b.eq            2f
        b               1b

2:
.if \bpc == 8
        ld1             {v0.8b}, [x2], #8
.else
        ld1             {v0.8h}, [x2], #16
.endif
        ld1             {v1.8h}, [x4], #16
        ld1             {v2.8h}, [x5], #16
        subs            x6,  x6,  #8
.if \bpc == 8
        ushll           v0.8h,  v0.8b,  #4     // u
.else
        shl             v0.8h,  v0.8h,  #4     // u
.endif
        sub             v1.8h,  v1.8h,  v0.8h  // t1 - u
        sub             v2.8h,  v2.8h,  v0.8h  // t2 - u
        ushll           v3.4s,  v0.4h,  #7     // u << 7
        ushll2          v4.4s,  v0.8h,  #7     // u << 7
        smlal           v3.4s,  v1.4h,  v30.4h // wt[0] * (t1 - u)
        smlal           v3.4s,  v2.4h,  v31.4h // wt[1] * (t2 - u)
        smlal2          v4.4s,  v1.8h,  v30.8h // wt[0] * (t1 - u)
        smlal2          v4.4s,  v2.8h,  v31.8h // wt[1] * (t2 - u)
.if \bpc == 8
        rshrn           v3.4h,  v3.4s,  #11
        rshrn2          v3.8h,  v4.4s,  #11
        sqxtun          v3.8b,  v3.8h
        st1             {v3.8b}, [x0], #8
.else
        sqrshrun        v3.4h,  v3.4s,  #11
        sqrshrun2       v3.8h,  v4.4s,  #11
        umin            v3.8h,  v3.8h,  v29.8h
        st1             {v3.8h}, [x0], #16
.endif
        b.gt            1b
0:
        ret
endfunc
.endm
