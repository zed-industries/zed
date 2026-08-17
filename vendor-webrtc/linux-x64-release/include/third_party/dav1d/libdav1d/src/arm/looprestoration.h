/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
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

#include "src/cpu.h"
#include "src/looprestoration.h"

#if ARCH_AARCH64
void BF(dav1d_wiener_filter7, neon)(pixel *p, const ptrdiff_t stride,
                                    const pixel (*left)[4], const pixel *lpf,
                                    const int w, int h,
                                    const LooprestorationParams *const params,
                                    const enum LrEdgeFlags edges
                                    HIGHBD_DECL_SUFFIX);
void BF(dav1d_wiener_filter5, neon)(pixel *p, const ptrdiff_t stride,
                                    const pixel (*left)[4], const pixel *lpf,
                                    const int w, int h,
                                    const LooprestorationParams *const params,
                                    const enum LrEdgeFlags edges
                                    HIGHBD_DECL_SUFFIX);
#else

// The 8bpc version calculates things slightly differently than the reference
// C version. That version calculates roughly this:
// int16_t sum = 0;
// for (int i = 0; i < 7; i++)
//     sum += src[idx] * fh[i];
// int16_t sum2 = (src[x] << 7) - (1 << (bitdepth + 6)) + rounding_off_h;
// sum = iclip(sum + sum2, INT16_MIN, INT16_MAX) >> round_bits_h;
// sum += 1 << (bitdepth + 6 - round_bits_h);
// Compared to the reference C version, this is the output of the first pass
// _subtracted_ by 1 << (bitdepth + 6 - round_bits_h) = 2048, i.e.
// with round_offset precompensated.
// The 16bpc version calculates things pretty much the same way as the
// reference C version, but with the end result subtracted by
// 1 << (bitdepth + 6 - round_bits_h).
void BF(dav1d_wiener_filter_h, neon)(int16_t *dst, const pixel (*left)[4],
                                     const pixel *src, const int16_t fh[8],
                                     const int w, const enum LrEdgeFlags edges
                                     HIGHBD_DECL_SUFFIX);
// This calculates things slightly differently than the reference C version.
// This version calculates roughly this:
// int32_t sum = 0;
// for (int i = 0; i < 7; i++)
//     sum += mid[idx] * fv[i];
// sum = (sum + rounding_off_v) >> round_bits_v;
// This function assumes that the width is a multiple of 8.
void BF(dav1d_wiener_filter_v, neon)(pixel *dst, int16_t **ptrs,
                                     const int16_t fv[8], const int w
                                     HIGHBD_DECL_SUFFIX);

void BF(dav1d_wiener_filter_hv, neon)(pixel *dst, const pixel (*left)[4],
                                      const pixel *src,
                                      const int16_t filter[2][8],
                                      const int w, const enum LrEdgeFlags edges,
                                      int16_t **ptrs
                                      HIGHBD_DECL_SUFFIX);

static void wiener_filter_neon(pixel *p, const ptrdiff_t stride,
                               const pixel (*left)[4], const pixel *lpf,
                               const int w, int h,
                               const LooprestorationParams *const params,
                               const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)
{
    ALIGN_STK_16(int16_t, hor, 6 * 384,);
    int16_t *ptrs[7], *rows[6];
    for (int i = 0; i < 6; i++)
        rows[i] = &hor[i * 384];
    const int16_t (*const filter)[8] = params->filter;
    const int16_t *fh = params->filter[0];
    const int16_t *fv = params->filter[1];
    const pixel *lpf_bottom = lpf + 6*PXSTRIDE(stride);

    const pixel *src = p;
    if (edges & LR_HAVE_TOP) {
        ptrs[0] = rows[0];
        ptrs[1] = rows[0];
        ptrs[2] = rows[1];
        ptrs[3] = rows[2];
        ptrs[4] = rows[2];
        ptrs[5] = rows[2];

        BF(dav1d_wiener_filter_h, neon)(rows[0], NULL, lpf, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);
        lpf += PXSTRIDE(stride);
        BF(dav1d_wiener_filter_h, neon)(rows[1], NULL, lpf, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);

        BF(dav1d_wiener_filter_h, neon)(rows[2], left, src, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto v1;

        ptrs[4] = ptrs[5] = rows[3];
        BF(dav1d_wiener_filter_h, neon)(rows[3], left, src, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto v2;

        ptrs[5] = rows[4];
        BF(dav1d_wiener_filter_h, neon)(rows[4], left, src, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto v3;
    } else {
        ptrs[0] = rows[0];
        ptrs[1] = rows[0];
        ptrs[2] = rows[0];
        ptrs[3] = rows[0];
        ptrs[4] = rows[0];
        ptrs[5] = rows[0];

        BF(dav1d_wiener_filter_h, neon)(rows[0], left, src, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto v1;

        ptrs[4] = ptrs[5] = rows[1];
        BF(dav1d_wiener_filter_h, neon)(rows[1], left, src, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto v2;

        ptrs[5] = rows[2];
        BF(dav1d_wiener_filter_h, neon)(rows[2], left, src, fh, w, edges
                                        HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto v3;

        ptrs[6] = rows[3];
        BF(dav1d_wiener_filter_hv, neon)(p, left, src, filter, w, edges, ptrs
                                         HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);
        p += PXSTRIDE(stride);

        if (--h <= 0)
            goto v3;

        ptrs[6] = rows[4];
        BF(dav1d_wiener_filter_hv, neon)(p, left, src, filter, w, edges, ptrs
                                         HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);
        p += PXSTRIDE(stride);

        if (--h <= 0)
            goto v3;
    }

    ptrs[6] = ptrs[5] + 384;
    do {
        BF(dav1d_wiener_filter_hv, neon)(p, left, src, filter, w, edges, ptrs
                                         HIGHBD_TAIL_SUFFIX);
        left++;
        src += PXSTRIDE(stride);
        p += PXSTRIDE(stride);
    } while (--h > 0);

    if (!(edges & LR_HAVE_BOTTOM))
        goto v3;

    BF(dav1d_wiener_filter_hv, neon)(p, NULL, lpf_bottom, filter, w, edges, ptrs
                                     HIGHBD_TAIL_SUFFIX);
    lpf_bottom += PXSTRIDE(stride);
    p += PXSTRIDE(stride);

    BF(dav1d_wiener_filter_hv, neon)(p, NULL, lpf_bottom, filter, w, edges, ptrs
                                     HIGHBD_TAIL_SUFFIX);
    p += PXSTRIDE(stride);
v1:
    BF(dav1d_wiener_filter_v, neon)(p, ptrs, fv, w HIGHBD_TAIL_SUFFIX);

    return;

v3:
    BF(dav1d_wiener_filter_v, neon)(p, ptrs, fv, w HIGHBD_TAIL_SUFFIX);
    p += PXSTRIDE(stride);
v2:
    BF(dav1d_wiener_filter_v, neon)(p, ptrs, fv, w HIGHBD_TAIL_SUFFIX);
    p += PXSTRIDE(stride);
    goto v1;
}
#endif

static void rotate_neon(int32_t **sumsq_ptrs, int16_t **sum_ptrs, int n) {
    int32_t *tmp32 = sumsq_ptrs[0];
    int16_t *tmp16 = sum_ptrs[0];
    for (int i = 0; i < n - 1; i++) {
        sumsq_ptrs[i] = sumsq_ptrs[i + 1];
        sum_ptrs[i] = sum_ptrs[i + 1];
    }
    sumsq_ptrs[n - 1] = tmp32;
    sum_ptrs[n - 1] = tmp16;
}
static void rotate5_x2_neon(int32_t **sumsq_ptrs, int16_t **sum_ptrs) {
    int32_t *tmp32[2];
    int16_t *tmp16[2];
    for (int i = 0; i < 2; i++) {
        tmp32[i] = sumsq_ptrs[i];
        tmp16[i] = sum_ptrs[i];
    }
    for (int i = 0; i < 3; i++) {
        sumsq_ptrs[i] = sumsq_ptrs[i + 2];
        sum_ptrs[i] = sum_ptrs[i + 2];
    }
    for (int i = 0; i < 2; i++) {
        sumsq_ptrs[3 + i] = tmp32[i];
        sum_ptrs[3 + i] = tmp16[i];
    }
}

void BF(dav1d_sgr_box3_row_h, neon)(int32_t *sumsq, int16_t *sum,
                                    const pixel (*left)[4],
                                    const pixel *src, const int w,
                                    const enum LrEdgeFlags edges);
void BF(dav1d_sgr_box5_row_h, neon)(int32_t *sumsq, int16_t *sum,
                                    const pixel (*left)[4],
                                    const pixel *src, const int w,
                                    const enum LrEdgeFlags edges);
void BF(dav1d_sgr_box35_row_h, neon)(int32_t *sumsq3, int16_t *sum3,
                                     int32_t *sumsq5, int16_t *sum5,
                                     const pixel (*left)[4],
                                     const pixel *src, const int w,
                                     const enum LrEdgeFlags edges);

#if ARCH_ARM
void dav1d_sgr_box3_row_v_neon(int32_t **sumsq, int16_t **sum,
                               int32_t *sumsq_out, int16_t *sum_out,
                               const int w);
void dav1d_sgr_box5_row_v_neon(int32_t **sumsq, int16_t **sum,
                               int32_t *sumsq_out, int16_t *sum_out,
                               const int w);
void dav1d_sgr_calc_row_ab1_neon(int32_t *AA, int16_t *BB, int w, int s,
                                 int bitdepth_max);
void dav1d_sgr_calc_row_ab2_neon(int32_t *AA, int16_t *BB, int w, int s,
                                 int bitdepth_max);
void BF(dav1d_sgr_finish_filter_row1, neon)(int16_t *tmp, const pixel *src,
                                            int32_t **A_ptrs, int16_t **B_ptrs,
                                            const int w);
void BF(dav1d_sgr_weighted_row1, neon)(pixel *dst, const int16_t *t1,
                                       const int w, const int wt
                                       HIGHBD_DECL_SUFFIX);
#else
void dav1d_sgr_box3_vert_neon(int32_t **sumsq, int16_t **sum,
                              int32_t *AA, int16_t *BB,
                              const int w, const int s,
                              const int bitdepth_max);
void dav1d_sgr_box5_vert_neon(int32_t **sumsq, int16_t **sum,
                              int32_t *AA, int16_t *BB,
                              const int w, const int s,
                              const int bitdepth_max);

void BF(dav1d_sgr_finish_weighted1, neon)(pixel *dst,
                                          int32_t **A_ptrs, int16_t **B_ptrs,
                                          const int w, const int w1
                                          HIGHBD_DECL_SUFFIX);
void BF(dav1d_sgr_finish_weighted2, neon)(pixel *dst, const ptrdiff_t stride,
                                          int32_t **A_ptrs, int16_t **B_ptrs,
                                          const int w, const int h,
                                          const int w1 HIGHBD_DECL_SUFFIX);

void BF(dav1d_sgr_finish_filter1_2rows, neon)(int16_t *tmp, const pixel *src,
                                              const ptrdiff_t src_stride,
                                              int32_t **A_ptrs,
                                              int16_t **B_ptrs,
                                              const int w, const int h);
#endif
void BF(dav1d_sgr_finish_filter2_2rows, neon)(int16_t *tmp, const pixel *src,
                                              const ptrdiff_t src_stride,
                                              int32_t **A_ptrs, int16_t **B_ptrs,
                                              const int w, const int h);
void BF(dav1d_sgr_weighted2, neon)(pixel *dst, const ptrdiff_t dst_stride,
                                   const int16_t *t1, const int16_t *t2,
                                   const int w, const int h,
                                   const int16_t wt[2] HIGHBD_DECL_SUFFIX);

static void sgr_box3_vert_neon(int32_t **sumsq, int16_t **sum,
                               int32_t *sumsq_out, int16_t *sum_out,
                               const int w, const int s, const int bitdepth_max) {
#if ARCH_ARM
    dav1d_sgr_box3_row_v_neon(sumsq, sum, sumsq_out, sum_out, w);
    dav1d_sgr_calc_row_ab1_neon(sumsq_out, sum_out, w, s, bitdepth_max);
#else
    // box3_v + calc_ab1
    dav1d_sgr_box3_vert_neon(sumsq, sum, sumsq_out, sum_out, w, s, bitdepth_max);
#endif
    rotate_neon(sumsq, sum, 3);
}

static void sgr_box5_vert_neon(int32_t **sumsq, int16_t **sum,
                               int32_t *sumsq_out, int16_t *sum_out,
                               const int w, const int s, const int bitdepth_max) {
#if ARCH_ARM
    dav1d_sgr_box5_row_v_neon(sumsq, sum, sumsq_out, sum_out, w);
    dav1d_sgr_calc_row_ab2_neon(sumsq_out, sum_out, w, s, bitdepth_max);
#else
    // box5_v + calc_ab2
    dav1d_sgr_box5_vert_neon(sumsq, sum, sumsq_out, sum_out, w, s, bitdepth_max);
#endif
    rotate5_x2_neon(sumsq, sum);
}

static void sgr_box3_hv_neon(int32_t **sumsq, int16_t **sum,
                             int32_t *AA, int16_t *BB,
                             const pixel (*left)[4],
                             const pixel *src, const int w,
                             const int s,
                             const enum LrEdgeFlags edges,
                             const int bitdepth_max) {
    BF(dav1d_sgr_box3_row_h, neon)(sumsq[2], sum[2], left, src, w, edges);
    sgr_box3_vert_neon(sumsq, sum, AA, BB, w, s, bitdepth_max);
}


static void sgr_finish1_neon(pixel **dst, const ptrdiff_t stride,
                             int32_t **A_ptrs, int16_t **B_ptrs, const int w,
                             const int w1 HIGHBD_DECL_SUFFIX) {
#if ARCH_ARM
    ALIGN_STK_16(int16_t, tmp, 384,);

    BF(dav1d_sgr_finish_filter_row1, neon)(tmp, *dst, A_ptrs, B_ptrs, w);
    BF(dav1d_sgr_weighted_row1, neon)(*dst, tmp, w, w1 HIGHBD_TAIL_SUFFIX);
#else
    BF(dav1d_sgr_finish_weighted1, neon)(*dst, A_ptrs, B_ptrs,
                                         w, w1 HIGHBD_TAIL_SUFFIX);
#endif
    *dst += PXSTRIDE(stride);
    rotate_neon(A_ptrs, B_ptrs, 3);
}

#define ARM_FILTER_OUT_STRIDE 384

static void sgr_finish2_neon(pixel **dst, const ptrdiff_t stride,
                             int32_t **A_ptrs, int16_t **B_ptrs,
                             const int w, const int h, const int w1
                             HIGHBD_DECL_SUFFIX) {
#if ARCH_ARM
    ALIGN_STK_16(int16_t, tmp, 2*ARM_FILTER_OUT_STRIDE,);

    BF(dav1d_sgr_finish_filter2_2rows, neon)(tmp, *dst, stride, A_ptrs, B_ptrs, w, h);
    BF(dav1d_sgr_weighted_row1, neon)(*dst, tmp, w, w1 HIGHBD_TAIL_SUFFIX);
    *dst += PXSTRIDE(stride);
    if (h > 1) {
        BF(dav1d_sgr_weighted_row1, neon)(*dst, tmp + FILTER_OUT_STRIDE, w, w1 HIGHBD_TAIL_SUFFIX);
        *dst += PXSTRIDE(stride);
    }
#else
    BF(dav1d_sgr_finish_weighted2, neon)(*dst, stride, A_ptrs, B_ptrs,
                                         w, h, w1 HIGHBD_TAIL_SUFFIX);
    *dst += 2*PXSTRIDE(stride);
#endif
    rotate_neon(A_ptrs, B_ptrs, 2);
}

static void sgr_finish_mix_neon(pixel **dst, const ptrdiff_t stride,
                                int32_t **A5_ptrs, int16_t **B5_ptrs,
                                int32_t **A3_ptrs, int16_t **B3_ptrs,
                                const int w, const int h,
                                const int w0, const int w1 HIGHBD_DECL_SUFFIX) {
    ALIGN_STK_16(int16_t, tmp5, 2*ARM_FILTER_OUT_STRIDE,);
    ALIGN_STK_16(int16_t, tmp3, 2*ARM_FILTER_OUT_STRIDE,);

    BF(dav1d_sgr_finish_filter2_2rows, neon)(tmp5, *dst, stride,
                                             A5_ptrs, B5_ptrs, w, h);
#if ARCH_ARM
    BF(dav1d_sgr_finish_filter_row1, neon)(tmp3, *dst, A3_ptrs, B3_ptrs, w);
    BF(dav1d_sgr_finish_filter_row1, neon)(tmp3 + FILTER_OUT_STRIDE,
                                           *dst + PXSTRIDE(stride),
                                           &A3_ptrs[1], &B3_ptrs[1], w);
#else
    BF(dav1d_sgr_finish_filter1_2rows, neon)(tmp3, *dst, stride,
                                             A3_ptrs, B3_ptrs, w, h);
#endif
    const int16_t wt[2] = { w0, w1 };
    BF(dav1d_sgr_weighted2, neon)(*dst, stride,
                                  tmp5, tmp3, w, h, wt HIGHBD_TAIL_SUFFIX);
    *dst += h*PXSTRIDE(stride);
    rotate_neon(A5_ptrs, B5_ptrs, 2);
    rotate_neon(A3_ptrs, B3_ptrs, 4);
}


static void sgr_filter_3x3_neon(pixel *dst, const ptrdiff_t stride,
                                const pixel (*left)[4], const pixel *lpf,
                                const int w, int h,
                                const LooprestorationParams *const params,
                                const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)
{
#define ARM_BUF_STRIDE (384 + 16)
    ALIGN_STK_16(int32_t, sumsq_buf, ARM_BUF_STRIDE * 3 + 16,);
    ALIGN_STK_16(int16_t, sum_buf, ARM_BUF_STRIDE * 3 + 16,);
    int32_t *sumsq_ptrs[3], *sumsq_rows[3];
    int16_t *sum_ptrs[3], *sum_rows[3];
    for (int i = 0; i < 3; i++) {
        sumsq_rows[i] = &sumsq_buf[i * ARM_BUF_STRIDE];
        sum_rows[i] = &sum_buf[i * ARM_BUF_STRIDE];
    }

    ALIGN_STK_16(int32_t, A_buf, ARM_BUF_STRIDE * 3 + 16,);
    ALIGN_STK_16(int16_t, B_buf, ARM_BUF_STRIDE * 3 + 16,);
    int32_t *A_ptrs[3];
    int16_t *B_ptrs[3];
    for (int i = 0; i < 3; i++) {
        A_ptrs[i] = &A_buf[i * ARM_BUF_STRIDE];
        B_ptrs[i] = &B_buf[i * ARM_BUF_STRIDE];
    }
    const pixel *src = dst;
    const pixel *lpf_bottom = lpf + 6*PXSTRIDE(stride);

    if (edges & LR_HAVE_TOP) {
        sumsq_ptrs[0] = sumsq_rows[0];
        sumsq_ptrs[1] = sumsq_rows[1];
        sumsq_ptrs[2] = sumsq_rows[2];
        sum_ptrs[0] = sum_rows[0];
        sum_ptrs[1] = sum_rows[1];
        sum_ptrs[2] = sum_rows[2];

        BF(dav1d_sgr_box3_row_h, neon)(sumsq_rows[0], sum_rows[0],
                                       NULL, lpf, w, edges);
        lpf += PXSTRIDE(stride);
        BF(dav1d_sgr_box3_row_h, neon)(sumsq_rows[1], sum_rows[1],
                                       NULL, lpf, w, edges);

        sgr_box3_hv_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                         left, src, w, params->sgr.s1, edges, BITDEPTH_MAX);
        left++;
        src += PXSTRIDE(stride);
        rotate_neon(A_ptrs, B_ptrs, 3);

        if (--h <= 0)
            goto vert_1;

        sgr_box3_hv_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                         left, src, w, params->sgr.s1, edges, BITDEPTH_MAX);
        left++;
        src += PXSTRIDE(stride);
        rotate_neon(A_ptrs, B_ptrs, 3);

        if (--h <= 0)
            goto vert_2;
    } else {
        sumsq_ptrs[0] = sumsq_rows[0];
        sumsq_ptrs[1] = sumsq_rows[0];
        sumsq_ptrs[2] = sumsq_rows[0];
        sum_ptrs[0] = sum_rows[0];
        sum_ptrs[1] = sum_rows[0];
        sum_ptrs[2] = sum_rows[0];

        BF(dav1d_sgr_box3_row_h, neon)(sumsq_rows[0], sum_rows[0],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box3_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                           w, params->sgr.s1, BITDEPTH_MAX);
        rotate_neon(A_ptrs, B_ptrs, 3);

        if (--h <= 0)
            goto vert_1;

        sumsq_ptrs[2] = sumsq_rows[1];
        sum_ptrs[2] = sum_rows[1];

        sgr_box3_hv_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                         left, src, w, params->sgr.s1, edges, BITDEPTH_MAX);
        left++;
        src += PXSTRIDE(stride);
        rotate_neon(A_ptrs, B_ptrs, 3);

        if (--h <= 0)
            goto vert_2;

        sumsq_ptrs[2] = sumsq_rows[2];
        sum_ptrs[2] = sum_rows[2];
    }

    do {
        sgr_box3_hv_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                         left, src, w, params->sgr.s1, edges, BITDEPTH_MAX);
        left++;
        src += PXSTRIDE(stride);

        sgr_finish1_neon(&dst, stride, A_ptrs, B_ptrs,
                         w, params->sgr.w1 HIGHBD_TAIL_SUFFIX);
    } while (--h > 0);

    if (!(edges & LR_HAVE_BOTTOM))
        goto vert_2;

    sgr_box3_hv_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                     NULL, lpf_bottom, w, params->sgr.s1, edges, BITDEPTH_MAX);
    lpf_bottom += PXSTRIDE(stride);

    sgr_finish1_neon(&dst, stride, A_ptrs, B_ptrs,
                     w, params->sgr.w1 HIGHBD_TAIL_SUFFIX);

    sgr_box3_hv_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                     NULL, lpf_bottom, w, params->sgr.s1, edges, BITDEPTH_MAX);

    sgr_finish1_neon(&dst, stride, A_ptrs, B_ptrs,
                     w, params->sgr.w1 HIGHBD_TAIL_SUFFIX);
    return;

vert_2:
    sumsq_ptrs[2] = sumsq_ptrs[1];
    sum_ptrs[2] = sum_ptrs[1];
    sgr_box3_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                       w, params->sgr.s1, BITDEPTH_MAX);

    sgr_finish1_neon(&dst, stride, A_ptrs, B_ptrs,
                     w, params->sgr.w1 HIGHBD_TAIL_SUFFIX);

output_1:
    sumsq_ptrs[2] = sumsq_ptrs[1];
    sum_ptrs[2] = sum_ptrs[1];
    sgr_box3_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                       w, params->sgr.s1, BITDEPTH_MAX);

    sgr_finish1_neon(&dst, stride, A_ptrs, B_ptrs,
                     w, params->sgr.w1 HIGHBD_TAIL_SUFFIX);
    return;

vert_1:
    sumsq_ptrs[2] = sumsq_ptrs[1];
    sum_ptrs[2] = sum_ptrs[1];
    sgr_box3_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[2], B_ptrs[2],
                       w, params->sgr.s1, BITDEPTH_MAX);
    rotate_neon(A_ptrs, B_ptrs, 3);
    goto output_1;
}

static void sgr_filter_5x5_neon(pixel *dst, const ptrdiff_t stride,
                                const pixel (*left)[4], const pixel *lpf,
                                const int w, int h,
                                const LooprestorationParams *const params,
                                const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)
{
    ALIGN_STK_16(int32_t, sumsq_buf, ARM_BUF_STRIDE * 5 + 16,);
    ALIGN_STK_16(int16_t, sum_buf, ARM_BUF_STRIDE * 5 + 16,);
    int32_t *sumsq_ptrs[5], *sumsq_rows[5];
    int16_t *sum_ptrs[5], *sum_rows[5];
    for (int i = 0; i < 5; i++) {
        sumsq_rows[i] = &sumsq_buf[i * ARM_BUF_STRIDE];
        sum_rows[i] = &sum_buf[i * ARM_BUF_STRIDE];
    }

    ALIGN_STK_16(int32_t, A_buf, ARM_BUF_STRIDE * 2 + 16,);
    ALIGN_STK_16(int16_t, B_buf, ARM_BUF_STRIDE * 2 + 16,);
    int32_t *A_ptrs[2];
    int16_t *B_ptrs[2];
    for (int i = 0; i < 2; i++) {
        A_ptrs[i] = &A_buf[i * ARM_BUF_STRIDE];
        B_ptrs[i] = &B_buf[i * ARM_BUF_STRIDE];
    }
    const pixel *src = dst;
    const pixel *lpf_bottom = lpf + 6*PXSTRIDE(stride);

    if (edges & LR_HAVE_TOP) {
        sumsq_ptrs[0] = sumsq_rows[0];
        sumsq_ptrs[1] = sumsq_rows[0];
        sumsq_ptrs[2] = sumsq_rows[1];
        sumsq_ptrs[3] = sumsq_rows[2];
        sumsq_ptrs[4] = sumsq_rows[3];
        sum_ptrs[0] = sum_rows[0];
        sum_ptrs[1] = sum_rows[0];
        sum_ptrs[2] = sum_rows[1];
        sum_ptrs[3] = sum_rows[2];
        sum_ptrs[4] = sum_rows[3];

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[0], sum_rows[0],
                                       NULL, lpf, w, edges);
        lpf += PXSTRIDE(stride);
        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[1], sum_rows[1],
                                       NULL, lpf, w, edges);

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[2], sum_rows[2],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto vert_1;

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[3], sum_rows[3],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);
        sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        rotate_neon(A_ptrs, B_ptrs, 2);

        if (--h <= 0)
            goto vert_2;

        // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
        // one of them to point at the previously unused rows[4].
        sumsq_ptrs[3] = sumsq_rows[4];
        sum_ptrs[3] = sum_rows[4];
    } else {
        sumsq_ptrs[0] = sumsq_rows[0];
        sumsq_ptrs[1] = sumsq_rows[0];
        sumsq_ptrs[2] = sumsq_rows[0];
        sumsq_ptrs[3] = sumsq_rows[0];
        sumsq_ptrs[4] = sumsq_rows[0];
        sum_ptrs[0] = sum_rows[0];
        sum_ptrs[1] = sum_rows[0];
        sum_ptrs[2] = sum_rows[0];
        sum_ptrs[3] = sum_rows[0];
        sum_ptrs[4] = sum_rows[0];

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[0], sum_rows[0],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto vert_1;

        sumsq_ptrs[4] = sumsq_rows[1];
        sum_ptrs[4] = sum_rows[1];

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[1], sum_rows[1],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        rotate_neon(A_ptrs, B_ptrs, 2);

        if (--h <= 0)
            goto vert_2;

        sumsq_ptrs[3] = sumsq_rows[2];
        sumsq_ptrs[4] = sumsq_rows[3];
        sum_ptrs[3] = sum_rows[2];
        sum_ptrs[4] = sum_rows[3];

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[2], sum_rows[2],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto odd;

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_rows[3], sum_rows[3],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        sgr_finish2_neon(&dst, stride, A_ptrs, B_ptrs,
                         w, 2, params->sgr.w0 HIGHBD_TAIL_SUFFIX);

        if (--h <= 0)
            goto vert_2;

        // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
        // one of them to point at the previously unused rows[4].
        sumsq_ptrs[3] = sumsq_rows[4];
        sum_ptrs[3] = sum_rows[4];
    }

    do {
        BF(dav1d_sgr_box5_row_h, neon)(sumsq_ptrs[3], sum_ptrs[3],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        if (--h <= 0)
            goto odd;

        BF(dav1d_sgr_box5_row_h, neon)(sumsq_ptrs[4], sum_ptrs[4],
                                       left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        sgr_finish2_neon(&dst, stride, A_ptrs, B_ptrs,
                         w, 2, params->sgr.w0 HIGHBD_TAIL_SUFFIX);
    } while (--h > 0);

    if (!(edges & LR_HAVE_BOTTOM))
        goto vert_2;

    BF(dav1d_sgr_box5_row_h, neon)(sumsq_ptrs[3], sum_ptrs[3],
                                   NULL, lpf_bottom, w, edges);
    lpf_bottom += PXSTRIDE(stride);
    BF(dav1d_sgr_box5_row_h, neon)(sumsq_ptrs[4], sum_ptrs[4],
                                   NULL, lpf_bottom, w, edges);

output_2:
    sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    sgr_finish2_neon(&dst, stride, A_ptrs, B_ptrs,
                     w, 2, params->sgr.w0 HIGHBD_TAIL_SUFFIX);
    return;

vert_2:
    // Duplicate the last row twice more
    sumsq_ptrs[3] = sumsq_ptrs[2];
    sumsq_ptrs[4] = sumsq_ptrs[2];
    sum_ptrs[3] = sum_ptrs[2];
    sum_ptrs[4] = sum_ptrs[2];
    goto output_2;

odd:
    // Copy the last row as padding once
    sumsq_ptrs[4] = sumsq_ptrs[3];
    sum_ptrs[4] = sum_ptrs[3];

    sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    sgr_finish2_neon(&dst, stride, A_ptrs, B_ptrs,
                     w, 2, params->sgr.w0 HIGHBD_TAIL_SUFFIX);

output_1:
    // Duplicate the last row twice more
    sumsq_ptrs[3] = sumsq_ptrs[2];
    sumsq_ptrs[4] = sumsq_ptrs[2];
    sum_ptrs[3] = sum_ptrs[2];
    sum_ptrs[4] = sum_ptrs[2];

    sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    // Output only one row
    sgr_finish2_neon(&dst, stride, A_ptrs, B_ptrs,
                     w, 1, params->sgr.w0 HIGHBD_TAIL_SUFFIX);
    return;

vert_1:
    // Copy the last row as padding once
    sumsq_ptrs[4] = sumsq_ptrs[3];
    sum_ptrs[4] = sum_ptrs[3];

    sgr_box5_vert_neon(sumsq_ptrs, sum_ptrs, A_ptrs[1], B_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    rotate_neon(A_ptrs, B_ptrs, 2);

    goto output_1;
}

static void sgr_filter_mix_neon(pixel *dst, const ptrdiff_t stride,
                                const pixel (*left)[4], const pixel *lpf,
                                const int w, int h,
                                const LooprestorationParams *const params,
                                const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)
{
    ALIGN_STK_16(int32_t, sumsq5_buf, ARM_BUF_STRIDE * 5 + 16,);
    ALIGN_STK_16(int16_t, sum5_buf, ARM_BUF_STRIDE * 5 + 16,);
    int32_t *sumsq5_ptrs[5], *sumsq5_rows[5];
    int16_t *sum5_ptrs[5], *sum5_rows[5];
    for (int i = 0; i < 5; i++) {
        sumsq5_rows[i] = &sumsq5_buf[i * ARM_BUF_STRIDE];
        sum5_rows[i] = &sum5_buf[i * ARM_BUF_STRIDE];
    }
    ALIGN_STK_16(int32_t, sumsq3_buf, ARM_BUF_STRIDE * 3 + 16,);
    ALIGN_STK_16(int16_t, sum3_buf, ARM_BUF_STRIDE * 3 + 16,);
    int32_t *sumsq3_ptrs[3], *sumsq3_rows[3];
    int16_t *sum3_ptrs[3], *sum3_rows[3];
    for (int i = 0; i < 3; i++) {
        sumsq3_rows[i] = &sumsq3_buf[i * ARM_BUF_STRIDE];
        sum3_rows[i] = &sum3_buf[i * ARM_BUF_STRIDE];
    }

    ALIGN_STK_16(int32_t, A5_buf, ARM_BUF_STRIDE * 2 + 16,);
    ALIGN_STK_16(int16_t, B5_buf, ARM_BUF_STRIDE * 2 + 16,);
    int32_t *A5_ptrs[2];
    int16_t *B5_ptrs[2];
    for (int i = 0; i < 2; i++) {
        A5_ptrs[i] = &A5_buf[i * ARM_BUF_STRIDE];
        B5_ptrs[i] = &B5_buf[i * ARM_BUF_STRIDE];
    }
    ALIGN_STK_16(int32_t, A3_buf, ARM_BUF_STRIDE * 4 + 16,);
    ALIGN_STK_16(int16_t, B3_buf, ARM_BUF_STRIDE * 4 + 16,);
    int32_t *A3_ptrs[4];
    int16_t *B3_ptrs[4];
    for (int i = 0; i < 4; i++) {
        A3_ptrs[i] = &A3_buf[i * ARM_BUF_STRIDE];
        B3_ptrs[i] = &B3_buf[i * ARM_BUF_STRIDE];
    }
    const pixel *src = dst;
    const pixel *lpf_bottom = lpf + 6*PXSTRIDE(stride);

    if (edges & LR_HAVE_TOP) {
        sumsq5_ptrs[0] = sumsq5_rows[0];
        sumsq5_ptrs[1] = sumsq5_rows[0];
        sumsq5_ptrs[2] = sumsq5_rows[1];
        sumsq5_ptrs[3] = sumsq5_rows[2];
        sumsq5_ptrs[4] = sumsq5_rows[3];
        sum5_ptrs[0] = sum5_rows[0];
        sum5_ptrs[1] = sum5_rows[0];
        sum5_ptrs[2] = sum5_rows[1];
        sum5_ptrs[3] = sum5_rows[2];
        sum5_ptrs[4] = sum5_rows[3];

        sumsq3_ptrs[0] = sumsq3_rows[0];
        sumsq3_ptrs[1] = sumsq3_rows[1];
        sumsq3_ptrs[2] = sumsq3_rows[2];
        sum3_ptrs[0] = sum3_rows[0];
        sum3_ptrs[1] = sum3_rows[1];
        sum3_ptrs[2] = sum3_rows[2];

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_rows[0], sum3_rows[0],
                                        sumsq5_rows[0], sum5_rows[0],
                                        NULL, lpf, w, edges);
        lpf += PXSTRIDE(stride);
        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_rows[1], sum3_rows[1],
                                        sumsq5_rows[1], sum5_rows[1],
                                        NULL, lpf, w, edges);

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_rows[2], sum3_rows[2],
                                        sumsq5_rows[2], sum5_rows[2],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        rotate_neon(A3_ptrs, B3_ptrs, 4);

        if (--h <= 0)
            goto vert_1;

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_ptrs[2], sum3_ptrs[2],
                                        sumsq5_rows[3], sum5_rows[3],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);
        sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        rotate_neon(A5_ptrs, B5_ptrs, 2);
        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        rotate_neon(A3_ptrs, B3_ptrs, 4);

        if (--h <= 0)
            goto vert_2;

        // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
        // one of them to point at the previously unused rows[4].
        sumsq5_ptrs[3] = sumsq5_rows[4];
        sum5_ptrs[3] = sum5_rows[4];
    } else {
        sumsq5_ptrs[0] = sumsq5_rows[0];
        sumsq5_ptrs[1] = sumsq5_rows[0];
        sumsq5_ptrs[2] = sumsq5_rows[0];
        sumsq5_ptrs[3] = sumsq5_rows[0];
        sumsq5_ptrs[4] = sumsq5_rows[0];
        sum5_ptrs[0] = sum5_rows[0];
        sum5_ptrs[1] = sum5_rows[0];
        sum5_ptrs[2] = sum5_rows[0];
        sum5_ptrs[3] = sum5_rows[0];
        sum5_ptrs[4] = sum5_rows[0];

        sumsq3_ptrs[0] = sumsq3_rows[0];
        sumsq3_ptrs[1] = sumsq3_rows[0];
        sumsq3_ptrs[2] = sumsq3_rows[0];
        sum3_ptrs[0] = sum3_rows[0];
        sum3_ptrs[1] = sum3_rows[0];
        sum3_ptrs[2] = sum3_rows[0];

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_rows[0], sum3_rows[0],
                                        sumsq5_rows[0], sum5_rows[0],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        rotate_neon(A3_ptrs, B3_ptrs, 4);

        if (--h <= 0)
            goto vert_1;

        sumsq5_ptrs[4] = sumsq5_rows[1];
        sum5_ptrs[4] = sum5_rows[1];

        sumsq3_ptrs[2] = sumsq3_rows[1];
        sum3_ptrs[2] = sum3_rows[1];

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_rows[1], sum3_rows[1],
                                        sumsq5_rows[1], sum5_rows[1],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        rotate_neon(A5_ptrs, B5_ptrs, 2);
        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        rotate_neon(A3_ptrs, B3_ptrs, 4);

        if (--h <= 0)
            goto vert_2;

        sumsq5_ptrs[3] = sumsq5_rows[2];
        sumsq5_ptrs[4] = sumsq5_rows[3];
        sum5_ptrs[3] = sum5_rows[2];
        sum5_ptrs[4] = sum5_rows[3];

        sumsq3_ptrs[2] = sumsq3_rows[2];
        sum3_ptrs[2] = sum3_rows[2];

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_rows[2], sum3_rows[2],
                                        sumsq5_rows[2], sum5_rows[2],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        rotate_neon(A3_ptrs, B3_ptrs, 4);

        if (--h <= 0)
            goto odd;

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_ptrs[2], sum3_ptrs[2],
                                        sumsq5_rows[3], sum5_rows[3],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        sgr_finish_mix_neon(&dst, stride, A5_ptrs, B5_ptrs, A3_ptrs, B3_ptrs,
                            w, 2, params->sgr.w0, params->sgr.w1
                            HIGHBD_TAIL_SUFFIX);

        if (--h <= 0)
            goto vert_2;

        // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
        // one of them to point at the previously unused rows[4].
        sumsq5_ptrs[3] = sumsq5_rows[4];
        sum5_ptrs[3] = sum5_rows[4];
    }

    do {
        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_ptrs[2], sum3_ptrs[2],
                                        sumsq5_ptrs[3], sum5_ptrs[3],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        rotate_neon(A3_ptrs, B3_ptrs, 4);

        if (--h <= 0)
            goto odd;

        BF(dav1d_sgr_box35_row_h, neon)(sumsq3_ptrs[2], sum3_ptrs[2],
                                        sumsq5_ptrs[4], sum5_ptrs[4],
                                        left, src, w, edges);
        left++;
        src += PXSTRIDE(stride);

        sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                           w, params->sgr.s0, BITDEPTH_MAX);
        sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                           w, params->sgr.s1, BITDEPTH_MAX);
        sgr_finish_mix_neon(&dst, stride, A5_ptrs, B5_ptrs, A3_ptrs, B3_ptrs,
                            w, 2, params->sgr.w0, params->sgr.w1
                            HIGHBD_TAIL_SUFFIX);
    } while (--h > 0);

    if (!(edges & LR_HAVE_BOTTOM))
        goto vert_2;

    BF(dav1d_sgr_box35_row_h, neon)(sumsq3_ptrs[2], sum3_ptrs[2],
                                    sumsq5_ptrs[3], sum5_ptrs[3],
                                    NULL, lpf_bottom, w, edges);
    lpf_bottom += PXSTRIDE(stride);
    sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                       w, params->sgr.s1, BITDEPTH_MAX);
    rotate_neon(A3_ptrs, B3_ptrs, 4);

    BF(dav1d_sgr_box35_row_h, neon)(sumsq3_ptrs[2], sum3_ptrs[2],
                                    sumsq5_ptrs[4], sum5_ptrs[4],
                                    NULL, lpf_bottom, w, edges);

output_2:
    sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                       w, params->sgr.s1, BITDEPTH_MAX);
    sgr_finish_mix_neon(&dst, stride, A5_ptrs, B5_ptrs, A3_ptrs, B3_ptrs,
                        w, 2, params->sgr.w0, params->sgr.w1
                        HIGHBD_TAIL_SUFFIX);
    return;

vert_2:
    // Duplicate the last row twice more
    sumsq5_ptrs[3] = sumsq5_ptrs[2];
    sumsq5_ptrs[4] = sumsq5_ptrs[2];
    sum5_ptrs[3] = sum5_ptrs[2];
    sum5_ptrs[4] = sum5_ptrs[2];

    sumsq3_ptrs[2] = sumsq3_ptrs[1];
    sum3_ptrs[2] = sum3_ptrs[1];
    sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                       w, params->sgr.s1, BITDEPTH_MAX);
    rotate_neon(A3_ptrs, B3_ptrs, 4);

    sumsq3_ptrs[2] = sumsq3_ptrs[1];
    sum3_ptrs[2] = sum3_ptrs[1];

    goto output_2;

odd:
    // Copy the last row as padding once
    sumsq5_ptrs[4] = sumsq5_ptrs[3];
    sum5_ptrs[4] = sum5_ptrs[3];

    sumsq3_ptrs[2] = sumsq3_ptrs[1];
    sum3_ptrs[2] = sum3_ptrs[1];

    sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                       w, params->sgr.s1, BITDEPTH_MAX);
    sgr_finish_mix_neon(&dst, stride, A5_ptrs, B5_ptrs, A3_ptrs, B3_ptrs,
                        w, 2, params->sgr.w0, params->sgr.w1
                        HIGHBD_TAIL_SUFFIX);

output_1:
    // Duplicate the last row twice more
    sumsq5_ptrs[3] = sumsq5_ptrs[2];
    sumsq5_ptrs[4] = sumsq5_ptrs[2];
    sum5_ptrs[3] = sum5_ptrs[2];
    sum5_ptrs[4] = sum5_ptrs[2];

    sumsq3_ptrs[2] = sumsq3_ptrs[1];
    sum3_ptrs[2] = sum3_ptrs[1];

    sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                       w, params->sgr.s1, BITDEPTH_MAX);
    rotate_neon(A3_ptrs, B3_ptrs, 4);
    // Output only one row
    sgr_finish_mix_neon(&dst, stride, A5_ptrs, B5_ptrs, A3_ptrs, B3_ptrs,
                        w, 1, params->sgr.w0, params->sgr.w1
                        HIGHBD_TAIL_SUFFIX);
    return;

vert_1:
    // Copy the last row as padding once
    sumsq5_ptrs[4] = sumsq5_ptrs[3];
    sum5_ptrs[4] = sum5_ptrs[3];

    sumsq3_ptrs[2] = sumsq3_ptrs[1];
    sum3_ptrs[2] = sum3_ptrs[1];

    sgr_box5_vert_neon(sumsq5_ptrs, sum5_ptrs, A5_ptrs[1], B5_ptrs[1],
                       w, params->sgr.s0, BITDEPTH_MAX);
    rotate_neon(A5_ptrs, B5_ptrs, 2);
    sgr_box3_vert_neon(sumsq3_ptrs, sum3_ptrs, A3_ptrs[3], B3_ptrs[3],
                       w, params->sgr.s1, BITDEPTH_MAX);
    rotate_neon(A3_ptrs, B3_ptrs, 4);

    goto output_1;
}


static ALWAYS_INLINE void loop_restoration_dsp_init_arm(Dav1dLoopRestorationDSPContext *const c, int bpc) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_ARM_CPU_FLAG_NEON)) return;

#if ARCH_AARCH64
    c->wiener[0] = BF(dav1d_wiener_filter7, neon);
    c->wiener[1] = BF(dav1d_wiener_filter5, neon);
#else
    c->wiener[0] = c->wiener[1] = wiener_filter_neon;
#endif
    if (BITDEPTH == 8 || bpc == 10) {
        c->sgr[0] = sgr_filter_5x5_neon;
        c->sgr[1] = sgr_filter_3x3_neon;
        c->sgr[2] = sgr_filter_mix_neon;
    }
}
