/*
 * Copyright (C) 2024 Niklas Haas
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

#ifndef SWSCALE_FORMAT_H
#define SWSCALE_FORMAT_H

#include "libavutil/csp.h"
#include "libavutil/pixdesc.h"

#include "swscale.h"

static inline int ff_q_isnan(const AVRational a)
{
    return !a.num && !a.den;
}

/* Like av_cmp_q but considers NaN == NaN */
static inline int ff_q_equal(const AVRational a, const AVRational b)
{
    return (ff_q_isnan(a) && ff_q_isnan(b)) || !av_cmp_q(a, b);
}

static inline int ff_cie_xy_equal(const AVCIExy a, const AVCIExy b)
{
    return ff_q_equal(a.x, b.x) && ff_q_equal(a.y, b.y);
}

static inline int ff_prim_equal(const AVPrimaryCoefficients *a,
                                const AVPrimaryCoefficients *b)
{
    return ff_cie_xy_equal(a->r, b->r) &&
           ff_cie_xy_equal(a->g, b->g) &&
           ff_cie_xy_equal(a->b, b->b);
}

enum {
    FIELD_TOP, /* top/even rows, or progressive */
    FIELD_BOTTOM, /* bottom/odd rows */
};

typedef struct SwsColor {
    enum AVColorPrimaries prim;
    enum AVColorTransferCharacteristic trc;
    AVPrimaryCoefficients gamut; /* mastering display gamut */
    AVRational min_luma;         /* minimum luminance in nits */
    AVRational max_luma;         /* maximum luminance in nits */
    AVRational frame_peak;       /* per-frame/scene peak luminance, or 0 */
    AVRational frame_avg;        /* per-frame/scene average luminance, or 0 */
} SwsColor;

static inline void ff_color_update_dynamic(SwsColor *dst, const SwsColor *src)
{
    dst->frame_peak = src->frame_peak;
    dst->frame_avg  = src->frame_avg;
}

/* Subset of AVFrame parameters that uniquely determine pixel representation */
typedef struct SwsFormat {
    int width, height;
    int interlaced;
    enum AVPixelFormat format;
    enum AVColorRange range;
    enum AVColorSpace csp;
    enum AVChromaLocation loc;
    const AVPixFmtDescriptor *desc; /* convenience */
    SwsColor color;
} SwsFormat;

/**
 * This function also sanitizes and strips the input data, removing irrelevant
 * fields for certain formats.
 */
SwsFormat ff_fmt_from_frame(const AVFrame *frame, int field);

static inline int ff_color_equal(const SwsColor *c1, const SwsColor *c2)
{
    return  c1->prim == c2->prim &&
            c1->trc  == c2->trc  &&
            ff_q_equal(c1->min_luma, c2->min_luma) &&
            ff_q_equal(c1->max_luma, c2->max_luma) &&
            ff_prim_equal(&c1->gamut, &c2->gamut);
}

/* Tests only the static components of a colorspace, ignoring dimensions and per-frame data */
static inline int ff_props_equal(const SwsFormat *fmt1, const SwsFormat *fmt2)
{
    return fmt1->interlaced == fmt2->interlaced &&
           fmt1->format     == fmt2->format     &&
           fmt1->range      == fmt2->range      &&
           fmt1->csp        == fmt2->csp        &&
           fmt1->loc        == fmt2->loc        &&
           ff_color_equal(&fmt1->color, &fmt2->color);
}

/* Tests only the static components of a colorspace, ignoring per-frame data */
static inline int ff_fmt_equal(const SwsFormat *fmt1, const SwsFormat *fmt2)
{
    return fmt1->width      == fmt2->width      &&
           fmt1->height     == fmt2->height     &&
           ff_props_equal(fmt1, fmt2);
}

static inline int ff_fmt_align(enum AVPixelFormat fmt)
{
    const AVPixFmtDescriptor *desc = av_pix_fmt_desc_get(fmt);
    if (desc->flags & AV_PIX_FMT_FLAG_BAYER) {
        return 2;
    } else {
        return 1 << desc->log2_chroma_h;
    }
}

int ff_test_fmt(const SwsFormat *fmt, int output);

/* Returns 1 if the formats are incomplete, 0 otherwise */
int ff_infer_colors(SwsColor *src, SwsColor *dst);

#endif /* SWSCALE_FORMAT_H */
