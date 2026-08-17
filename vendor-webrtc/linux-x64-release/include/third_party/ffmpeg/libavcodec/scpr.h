/*
 * ScreenPressor decoder
 *
 * Copyright (c) 2017 Paul B Mahol
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

#ifndef AVCODEC_SCPR_H
#define AVCODEC_SCPR_H

#include "avcodec.h"
#include "bytestream.h"
#include "scpr3.h"

typedef struct RangeCoder {
    uint32_t   code;
    uint32_t   range;
    uint32_t   code1;
} RangeCoder;

typedef struct PixelModel {
    uint32_t    freq[256];
    uint32_t    lookup[16];
    uint32_t    total_freq;
} PixelModel;

typedef struct SCPRContext {
    int             version;
    AVFrame        *last_frame;
    AVFrame        *current_frame;
    GetByteContext  gb;
    RangeCoder      rc;
    PixelModel      pixel_model[3][4096];
    uint32_t        op_model[6][7];
    uint32_t        run_model[6][257];
    uint32_t        range_model[257];
    uint32_t        count_model[257];
    uint32_t        fill_model[6];
    uint32_t        sxy_model[4][17];
    uint32_t        mv_model[2][513];
    uint32_t        nbx, nby;
    uint32_t        nbcount;
    uint32_t       *blocks;
    uint32_t        cbits;
    int             cxshift;

    PixelModel3     pixel_model3[3][4096];
    RunModel3       run_model3[6];
    RunModel3       range_model3;
    RunModel3       count_model3;
    FillModel3      fill_model3;
    SxyModel3       sxy_model3[4];
    MVModel3        mv_model3[2];
    OpModel3        op_model3[6];

    int           (*get_freq)(RangeCoder *rc, uint32_t total_freq, uint32_t *freq);
    int           (*decode)(GetByteContext *gb, RangeCoder *rc, uint32_t cumFreq, uint32_t freq, uint32_t total_freq);
} SCPRContext;

static int decode_run_i(AVCodecContext *avctx, uint32_t ptype, int run,
                        int *px, int *py, uint32_t clr, uint32_t *dst,
                        int linesize, uint32_t *plx, uint32_t *ply,
                        uint32_t backstep, int off, int *cx, int *cx1)
{
    uint32_t r, g, b;
    int z;
    int x = *px,
        y = *py;
    uint32_t lx = *plx,
             ly = *ply;

    if (y >= avctx->height)
        return AVERROR_INVALIDDATA;

    switch (ptype) {
    case 0:
        while (run-- > 0) {
            dst[y * linesize + x] = clr;
            lx = x;
            ly = y;
            (x)++;
            if (x >= avctx->width) {
                x = 0;
                (y)++;
                if (y >= avctx->height && run)
                    return AVERROR_INVALIDDATA;
            }
        }
        break;
    case 1:
        while (run-- > 0) {
            dst[y * linesize + x] = dst[ly * linesize + lx];
            lx = x;
            ly = y;
            (x)++;
            if (x >= avctx->width) {
                x = 0;
                (y)++;
                if (y >= avctx->height && run)
                    return AVERROR_INVALIDDATA;
            }
        }
        clr = dst[ly * linesize + lx];
        break;
    case 2:
        if (y < 1)
            return AVERROR_INVALIDDATA;

        while (run-- > 0) {
            clr = dst[y * linesize + x + off + 1];
            dst[y * linesize + x] = clr;
            lx = x;
            ly = y;
            (x)++;
            if (x >= avctx->width) {
                x = 0;
                (y)++;
                if (y >= avctx->height && run)
                    return AVERROR_INVALIDDATA;
            }
        }
        break;
    case 4:
        if (y < 1 || (y == 1 && x == 0))
            return AVERROR_INVALIDDATA;

        while (run-- > 0) {
            uint8_t *odst = (uint8_t *)dst;
            int off1 = (ly * linesize + lx) * 4;
            int off2 = ((y * linesize + x) + off) * 4;

            if (x == 0) {
                z = backstep * 4;
            } else {
                z = 0;
            }

            r = odst[off1] +
                odst[off2 + 4] -
                odst[off2 - z ];
            g = odst[off1 + 1] +
                odst[off2 + 5] -
                odst[off2 - z  + 1];
            b = odst[off1 + 2] +
                odst[off2 + 6] -
                odst[off2 - z  + 2];
            clr = ((b & 0xFF) << 16) + ((g & 0xFF) << 8) + (r & 0xFF);
            dst[y * linesize + x] = clr;
            lx = x;
            ly = y;
            (x)++;
            if (x >= avctx->width) {
                x = 0;
                (y)++;
                if (y >= avctx->height && run)
                    return AVERROR_INVALIDDATA;
            }
        }
        break;
    case 5:
        if (y < 1 || (y == 1 && x == 0))
            return AVERROR_INVALIDDATA;

        while (run-- > 0) {
            if (x == 0) {
                z = backstep;
            } else {
                z = 0;
            }

            clr = dst[y * linesize + x + off - z];
            dst[y * linesize + x] = clr;
            lx = x;
            ly = y;
            (x)++;
            if (x >= avctx->width) {
                x = 0;
                (y)++;
                if (y >= avctx->height && run)
                    return AVERROR_INVALIDDATA;
            }
        }
        break;
    }

    *px = x;
    *py = y;
    *plx= lx;
    *ply= ly;

    if (avctx->bits_per_coded_sample == 16) {
        *cx1 = (clr & 0x3F00) >> 2;
        *cx = (clr & 0x3FFFFF) >> 16;
    } else {
        *cx1 = (clr & 0xFC00) >> 4;
        *cx = (clr & 0xFFFFFF) >> 18;
    }

    return 0;
}

static int decode_run_p(AVCodecContext *avctx, uint32_t ptype, int run,
                        int x, int y, uint32_t clr,
                        uint32_t *dst, uint32_t *prev,
                        int linesize, int plinesize,
                        uint32_t *bx, uint32_t *by,
                        uint32_t backstep, int sx1, int sx2,
                        int *cx, int *cx1)
{
    uint32_t r, g, b;
    int z;

    switch (ptype) {
    case 0:
        while (run-- > 0) {
            if (*by >= avctx->height)
                return AVERROR_INVALIDDATA;

            dst[*by * linesize + *bx] = clr;
            (*bx)++;
            if (*bx >= x * 16 + sx2 || *bx >= avctx->width) {
                *bx = x * 16 + sx1;
                (*by)++;
            }
        }
        break;
    case 1:
        while (run-- > 0) {
            if (*bx == 0) {
                if (*by < 1)
                    return AVERROR_INVALIDDATA;
                z = backstep;
            } else {
                z = 0;
            }

            if (*by >= avctx->height)
                return AVERROR_INVALIDDATA;

            clr = dst[*by * linesize + *bx - 1 - z];
            dst[*by * linesize + *bx] = clr;
            (*bx)++;
            if (*bx >= x * 16 + sx2 || *bx >= avctx->width) {
                *bx = x * 16 + sx1;
                (*by)++;
            }
        }
        break;
    case 2:
        while (run-- > 0) {
            if (*by < 1 || *by >= avctx->height)
                return AVERROR_INVALIDDATA;

            clr = dst[(*by - 1) * linesize + *bx];
            dst[*by * linesize + *bx] = clr;
            (*bx)++;
            if (*bx >= x * 16 + sx2 || *bx >= avctx->width) {
                *bx = x * 16 + sx1;
                (*by)++;
            }
        }
        break;
    case 3:
        while (run-- > 0) {
            if (*by >= avctx->height)
                return AVERROR_INVALIDDATA;

            clr = prev[*by * plinesize + *bx];
            dst[*by * linesize + *bx] = clr;
            (*bx)++;
            if (*bx >= x * 16 + sx2 || *bx >= avctx->width) {
                *bx = x * 16 + sx1;
                (*by)++;
            }
        }
        break;
    case 4:
        while (run-- > 0) {
            uint8_t *odst = (uint8_t *)dst;

            if (*by < 1 || *by >= avctx->height)
                return AVERROR_INVALIDDATA;

            if (*bx == 0) {
                if (*by < 2)
                    return AVERROR_INVALIDDATA;
                z = backstep;
            } else {
                z = 0;
            }

            r = odst[((*by - 1) * linesize + *bx) * 4] +
                odst[(*by * linesize + *bx - 1 - z) * 4] -
                odst[((*by - 1) * linesize + *bx - 1 - z) * 4];
            g = odst[((*by - 1) * linesize + *bx) * 4 + 1] +
                odst[(*by * linesize + *bx - 1 - z) * 4 + 1] -
                odst[((*by - 1) * linesize + *bx - 1 - z) * 4 + 1];
            b = odst[((*by - 1) * linesize + *bx) * 4 + 2] +
                odst[(*by * linesize + *bx - 1 - z) * 4 + 2] -
                odst[((*by - 1) * linesize + *bx - 1 - z) * 4 + 2];
            clr = ((b & 0xFF) << 16) + ((g & 0xFF) << 8) + (r & 0xFF);
            dst[*by * linesize + *bx] = clr;
            (*bx)++;
            if (*bx >= x * 16 + sx2 || *bx >= avctx->width) {
                *bx = x * 16 + sx1;
                (*by)++;
            }
        }
        break;
    case 5:
        while (run-- > 0) {
            if (*by < 1 || *by >= avctx->height)
                return AVERROR_INVALIDDATA;

            if (*bx == 0) {
                if (*by < 2)
                    return AVERROR_INVALIDDATA;
                z = backstep;
            } else {
                z = 0;
            }

            clr = dst[(*by - 1) * linesize + *bx - 1 - z];
            dst[*by * linesize + *bx] = clr;
            (*bx)++;
            if (*bx >= x * 16 + sx2 || *bx >= avctx->width) {
                *bx = x * 16 + sx1;
                (*by)++;
            }
        }
        break;
    }

    if (avctx->bits_per_coded_sample == 16) {
        *cx1 = (clr & 0x3F00) >> 2;
        *cx = (clr & 0x3FFFFF) >> 16;
    } else {
        *cx1 = (clr & 0xFC00) >> 4;
        *cx = (clr & 0xFFFFFF) >> 18;
    }

    return 0;
}

#endif /* AVCODEC_SCPR_H */
