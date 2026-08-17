/*
 * Copyright © 2018, VideoLAN and dav1d authors
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
#include "src/ipred.h"

decl_angular_ipred_fn(BF(dav1d_ipred_dc, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_dc_128, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_dc_top, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_dc_left, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_h, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_v, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_paeth, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth_v, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth_h, neon));
decl_angular_ipred_fn(BF(dav1d_ipred_filter, neon));

decl_cfl_pred_fn(BF(dav1d_ipred_cfl, neon));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_128, neon));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_top, neon));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_left, neon));

decl_cfl_ac_fn(BF(dav1d_ipred_cfl_ac_420, neon));
decl_cfl_ac_fn(BF(dav1d_ipred_cfl_ac_422, neon));
decl_cfl_ac_fn(BF(dav1d_ipred_cfl_ac_444, neon));

decl_pal_pred_fn(BF(dav1d_pal_pred, neon));

#if ARCH_AARCH64
void BF(dav1d_ipred_z1_upsample_edge, neon)(pixel *out, const int hsz,
                                            const pixel *const in,
                                            const int end HIGHBD_DECL_SUFFIX);
void BF(dav1d_ipred_z1_filter_edge, neon)(pixel *out, const int sz,
                                          const pixel *const in,
                                          const int end, const int strength);
void BF(dav1d_ipred_pixel_set, neon)(pixel *out, const pixel px,
                                     const int n);
void BF(dav1d_ipred_z1_fill1, neon)(pixel *dst, ptrdiff_t stride,
                                    const pixel *const top, const int width,
                                    const int height, const int dx,
                                    const int max_base_x);
void BF(dav1d_ipred_z1_fill2, neon)(pixel *dst, ptrdiff_t stride,
                                    const pixel *const top, const int width,
                                    const int height, const int dx,
                                    const int max_base_x);

static void ipred_z1_neon(pixel *dst, const ptrdiff_t stride,
                          const pixel *const topleft_in,
                          const int width, const int height, int angle,
                          const int max_width, const int max_height
                          HIGHBD_DECL_SUFFIX)
{
    const int is_sm = (angle >> 9) & 0x1;
    const int enable_intra_edge_filter = angle >> 10;
    angle &= 511;
    int dx = dav1d_dr_intra_derivative[angle >> 1];
    pixel top_out[64 + 64 + (64+15)*2 + 16];
    int max_base_x;
    const int upsample_above = enable_intra_edge_filter ?
        get_upsample(width + height, 90 - angle, is_sm) : 0;
    if (upsample_above) {
        BF(dav1d_ipred_z1_upsample_edge, neon)(top_out, width + height,
                                               topleft_in,
                                               width + imin(width, height)
                                               HIGHBD_TAIL_SUFFIX);
        max_base_x = 2 * (width + height) - 2;
        dx <<= 1;
    } else {
        const int filter_strength = enable_intra_edge_filter ?
            get_filter_strength(width + height, 90 - angle, is_sm) : 0;
        if (filter_strength) {
            BF(dav1d_ipred_z1_filter_edge, neon)(top_out, width + height,
                                                 topleft_in,
                                                 width + imin(width, height),
                                                 filter_strength);
            max_base_x = width + height - 1;
        } else {
            max_base_x = width + imin(width, height) - 1;
            memcpy(top_out, &topleft_in[1], (max_base_x + 1) * sizeof(pixel));
        }
    }
    const int base_inc = 1 + upsample_above;
    int pad_pixels = width + 15; // max(dx >> 6) == 15
    BF(dav1d_ipred_pixel_set, neon)(&top_out[max_base_x + 1],
                                    top_out[max_base_x], pad_pixels * base_inc);
    if (upsample_above)
        BF(dav1d_ipred_z1_fill2, neon)(dst, stride, top_out, width, height,
                                       dx, max_base_x);
    else
        BF(dav1d_ipred_z1_fill1, neon)(dst, stride, top_out, width, height,
                                       dx, max_base_x);
}

void BF(dav1d_ipred_reverse, neon)(pixel *dst, const pixel *const src,
                                   const int n);

void BF(dav1d_ipred_z2_upsample_edge, neon)(pixel *out, const int sz,
                                            const pixel *const in
                                            HIGHBD_DECL_SUFFIX);

void BF(dav1d_ipred_z2_fill1, neon)(pixel *dst, ptrdiff_t stride,
                                    const pixel *const top,
                                    const pixel *const left,
                                    const int width, const int height,
                                    const int dx, const int dy);
void BF(dav1d_ipred_z2_fill2, neon)(pixel *dst, ptrdiff_t stride,
                                    const pixel *const top,
                                    const pixel *const left,
                                    const int width, const int height,
                                    const int dx, const int dy);
void BF(dav1d_ipred_z2_fill3, neon)(pixel *dst, ptrdiff_t stride,
                                    const pixel *const top,
                                    const pixel *const left,
                                    const int width, const int height,
                                    const int dx, const int dy);

static void ipred_z2_neon(pixel *dst, const ptrdiff_t stride,
                          const pixel *const topleft_in,
                          const int width, const int height, int angle,
                          const int max_width, const int max_height
                          HIGHBD_DECL_SUFFIX)
{
    const int is_sm = (angle >> 9) & 0x1;
    const int enable_intra_edge_filter = angle >> 10;
    angle &= 511;
    assert(angle > 90 && angle < 180);
    int dy = dav1d_dr_intra_derivative[(angle - 90) >> 1];
    int dx = dav1d_dr_intra_derivative[(180 - angle) >> 1];
    const int upsample_left = enable_intra_edge_filter ?
        get_upsample(width + height, 180 - angle, is_sm) : 0;
    const int upsample_above = enable_intra_edge_filter ?
        get_upsample(width + height, angle - 90, is_sm) : 0;
    pixel buf[3*(64+1)];
    pixel *left = &buf[2*(64+1)];
    // The asm can underread below the start of top[] and left[]; to avoid
    // surprising behaviour, make sure this is within the allocated stack space.
    pixel *top = &buf[1*(64+1)];
    pixel *flipped = &buf[0*(64+1)];

    if (upsample_above) {
        BF(dav1d_ipred_z2_upsample_edge, neon)(top, width, topleft_in
                                               HIGHBD_TAIL_SUFFIX);
        dx <<= 1;
    } else {
        const int filter_strength = enable_intra_edge_filter ?
            get_filter_strength(width + height, angle - 90, is_sm) : 0;

        if (filter_strength) {
            BF(dav1d_ipred_z1_filter_edge, neon)(&top[1], imin(max_width, width),
                                                 topleft_in, width,
                                                 filter_strength);
            if (max_width < width)
                memcpy(&top[1 + max_width], &topleft_in[1 + max_width],
                       (width - max_width) * sizeof(pixel));
        } else {
            pixel_copy(&top[1], &topleft_in[1], width);
        }
    }
    if (upsample_left) {
        flipped[0] = topleft_in[0];
        BF(dav1d_ipred_reverse, neon)(&flipped[1], &topleft_in[0],
                                      height);
        BF(dav1d_ipred_z2_upsample_edge, neon)(left, height, flipped
                                               HIGHBD_TAIL_SUFFIX);
        dy <<= 1;
    } else {
        const int filter_strength = enable_intra_edge_filter ?
            get_filter_strength(width + height, 180 - angle, is_sm) : 0;

        if (filter_strength) {
            flipped[0] = topleft_in[0];
            BF(dav1d_ipred_reverse, neon)(&flipped[1], &topleft_in[0],
                                          height);
            BF(dav1d_ipred_z1_filter_edge, neon)(&left[1], imin(max_height, height),
                                                 flipped, height,
                                                 filter_strength);
            if (max_height < height)
                memcpy(&left[1 + max_height], &flipped[1 + max_height],
                       (height - max_height) * sizeof(pixel));
        } else {
            BF(dav1d_ipred_reverse, neon)(&left[1], &topleft_in[0],
                                          height);
        }
    }
    top[0] = left[0] = *topleft_in;

    assert(!(upsample_above && upsample_left));
    if (!upsample_above && !upsample_left) {
        BF(dav1d_ipred_z2_fill1, neon)(dst, stride, top, left, width, height,
                                       dx, dy);
    } else if (upsample_above) {
        BF(dav1d_ipred_z2_fill2, neon)(dst, stride, top, left, width, height,
                                       dx, dy);
    } else /*if (upsample_left)*/ {
        BF(dav1d_ipred_z2_fill3, neon)(dst, stride, top, left, width, height,
                                       dx, dy);
    }
}

void BF(dav1d_ipred_z3_fill1, neon)(pixel *dst, ptrdiff_t stride,
                                    const pixel *const left, const int width,
                                    const int height, const int dy,
                                    const int max_base_y);
void BF(dav1d_ipred_z3_fill2, neon)(pixel *dst, ptrdiff_t stride,
                                    const pixel *const left, const int width,
                                    const int height, const int dy,
                                    const int max_base_y);

static void ipred_z3_neon(pixel *dst, const ptrdiff_t stride,
                          const pixel *const topleft_in,
                          const int width, const int height, int angle,
                          const int max_width, const int max_height
                          HIGHBD_DECL_SUFFIX)
{
    const int is_sm = (angle >> 9) & 0x1;
    const int enable_intra_edge_filter = angle >> 10;
    angle &= 511;
    assert(angle > 180);
    int dy = dav1d_dr_intra_derivative[(270 - angle) >> 1];
    pixel flipped[64 + 64 + 16];
    pixel left_out[64 + 64 + (64+15)*2];
    int max_base_y;
    const int upsample_left = enable_intra_edge_filter ?
        get_upsample(width + height, angle - 180, is_sm) : 0;
    if (upsample_left) {
        flipped[0] = topleft_in[0];
        BF(dav1d_ipred_reverse, neon)(&flipped[1], &topleft_in[0],
                                      height + imax(width, height));
        BF(dav1d_ipred_z1_upsample_edge, neon)(left_out, width + height,
                                               flipped,
                                               height + imin(width, height)
                                               HIGHBD_TAIL_SUFFIX);
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        const int filter_strength = enable_intra_edge_filter ?
            get_filter_strength(width + height, angle - 180, is_sm) : 0;

        if (filter_strength) {
            flipped[0] = topleft_in[0];
            BF(dav1d_ipred_reverse, neon)(&flipped[1], &topleft_in[0],
                                          height + imax(width, height));
            BF(dav1d_ipred_z1_filter_edge, neon)(left_out, width + height,
                                                 flipped,
                                                 height + imin(width, height),
                                                 filter_strength);
            max_base_y = width + height - 1;
        } else {
            BF(dav1d_ipred_reverse, neon)(left_out, &topleft_in[0],
                                          height + imin(width, height));
            max_base_y = height + imin(width, height) - 1;
        }
    }
    const int base_inc = 1 + upsample_left;
    // The tbx based implementation needs left[] to have 64 bytes intitialized,
    // the other implementation can read height + max(dy >> 6) past the end.
    int pad_pixels = imax(64 - max_base_y - 1, height + 15);

    BF(dav1d_ipred_pixel_set, neon)(&left_out[max_base_y + 1],
                                    left_out[max_base_y], pad_pixels * base_inc);
    if (upsample_left)
        BF(dav1d_ipred_z3_fill2, neon)(dst, stride, left_out, width, height,
                                       dy, max_base_y);
    else
        BF(dav1d_ipred_z3_fill1, neon)(dst, stride, left_out, width, height,
                                       dy, max_base_y);
}
#endif

static ALWAYS_INLINE void intra_pred_dsp_init_arm(Dav1dIntraPredDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_ARM_CPU_FLAG_NEON)) return;

    c->intra_pred[DC_PRED]       = BF(dav1d_ipred_dc, neon);
    c->intra_pred[DC_128_PRED]   = BF(dav1d_ipred_dc_128, neon);
    c->intra_pred[TOP_DC_PRED]   = BF(dav1d_ipred_dc_top, neon);
    c->intra_pred[LEFT_DC_PRED]  = BF(dav1d_ipred_dc_left, neon);
    c->intra_pred[HOR_PRED]      = BF(dav1d_ipred_h, neon);
    c->intra_pred[VERT_PRED]     = BF(dav1d_ipred_v, neon);
    c->intra_pred[PAETH_PRED]    = BF(dav1d_ipred_paeth, neon);
    c->intra_pred[SMOOTH_PRED]   = BF(dav1d_ipred_smooth, neon);
    c->intra_pred[SMOOTH_V_PRED] = BF(dav1d_ipred_smooth_v, neon);
    c->intra_pred[SMOOTH_H_PRED] = BF(dav1d_ipred_smooth_h, neon);
#if ARCH_AARCH64
    c->intra_pred[Z1_PRED]       = ipred_z1_neon;
    c->intra_pred[Z2_PRED]       = ipred_z2_neon;
    c->intra_pred[Z3_PRED]       = ipred_z3_neon;
#endif
    c->intra_pred[FILTER_PRED]   = BF(dav1d_ipred_filter, neon);

    c->cfl_pred[DC_PRED]         = BF(dav1d_ipred_cfl, neon);
    c->cfl_pred[DC_128_PRED]     = BF(dav1d_ipred_cfl_128, neon);
    c->cfl_pred[TOP_DC_PRED]     = BF(dav1d_ipred_cfl_top, neon);
    c->cfl_pred[LEFT_DC_PRED]    = BF(dav1d_ipred_cfl_left, neon);

    c->cfl_ac[DAV1D_PIXEL_LAYOUT_I420 - 1] = BF(dav1d_ipred_cfl_ac_420, neon);
    c->cfl_ac[DAV1D_PIXEL_LAYOUT_I422 - 1] = BF(dav1d_ipred_cfl_ac_422, neon);
    c->cfl_ac[DAV1D_PIXEL_LAYOUT_I444 - 1] = BF(dav1d_ipred_cfl_ac_444, neon);

    c->pal_pred                  = BF(dav1d_pal_pred, neon);
}
