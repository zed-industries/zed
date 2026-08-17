/*
 * Copyright (c) 2015 Muhammad Faiz <mfcc64@gmail.com>
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVFILTER_SHOWCQT_H
#define AVFILTER_SHOWCQT_H

#include "libavutil/tx.h"
#include "avfilter.h"

typedef struct Coeffs {
    float *val;
    int start, len;
} Coeffs;

typedef struct RGBFloat {
    float r, g, b;
} RGBFloat;

typedef struct YUVFloat {
    float y, u, v;
} YUVFloat;

typedef union {
    RGBFloat rgb;
    YUVFloat yuv;
} ColorFloat;

typedef struct ShowCQTContext {
    const AVClass       *class;
    AVFilterContext     *ctx;
    AVFrame             *axis_frame;
    AVFrame             *sono_frame;
    enum AVPixelFormat  format;
    int                 sono_idx;
    int                 sono_count;
    int                 step;
    AVRational          step_frac;
    int                 remaining_frac;
    int                 remaining_fill;
    int                 remaining_fill_max;
    int64_t             next_pts;
    double              *freq;
    AVTXContext         *fft_ctx;
    av_tx_fn            tx_fn;
    Coeffs              *coeffs;
    AVComplexFloat      *fft_data;
    AVComplexFloat      *fft_input;
    AVComplexFloat      *fft_result;
    AVComplexFloat      *cqt_result;
    float               *attack_data;
    int                 fft_bits;
    int                 fft_len;
    int                 cqt_len;
    int                 cqt_align;
    ColorFloat          *c_buf;
    float               *h_buf;
    float               *rcp_h_buf;
    float               *sono_v_buf;
    float               *bar_v_buf;
    float               cmatrix[3][3];
    float               cscheme_v[6];
    /* callback */
    void                (*cqt_calc)(AVComplexFloat *dst, const AVComplexFloat *src, const Coeffs *coeffs,
                                    int len, int fft_len);
    void                (*permute_coeffs)(float *v, int len);
    void                (*draw_bar)(AVFrame *out, const float *h, const float *rcp_h,
                                    const ColorFloat *c, int bar_h, float bar_t);
    void                (*draw_axis)(AVFrame *out, AVFrame *axis, const ColorFloat *c, int off);
    void                (*draw_sono)(AVFrame *out, AVFrame *sono, int off, int idx);
    void                (*update_sono)(AVFrame *sono, const ColorFloat *c, int idx);
    /* performance debugging */
    int64_t             fft_time;
    int64_t             cqt_time;
    int64_t             process_cqt_time;
    int64_t             update_sono_time;
    int64_t             alloc_time;
    int64_t             bar_time;
    int64_t             axis_time;
    int64_t             sono_time;
    /* option */
    int                 width, height;
    AVRational          rate;
    int                 bar_h;
    int                 axis_h;
    int                 sono_h;
    int                 fullhd; /* deprecated */
    char                *sono_v;
    char                *bar_v;
    float               sono_g;
    float               bar_g;
    float               bar_t;
    double              timeclamp;
    double              attack;
    double              basefreq;
    double              endfreq;
    float               coeffclamp; /* deprecated - ignored */
    char                *tlength;
    int                 count;
    int                 fcount;
    char                *fontfile;
    char                *font;
    char                *fontcolor;
    char                *axisfile;
    int                 axis;
    int                 csp;
    char                *cscheme;
} ShowCQTContext;

void ff_showcqt_init_x86(ShowCQTContext *s);

#endif
