/*
 * Copyright (c) 2012-2013 Oka Motofumi (chikuzen.mo at gmail dot com)
 * Copyright (c) 2015 Paul B Mahol
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
#ifndef AVFILTER_CONVOLUTION_H
#define AVFILTER_CONVOLUTION_H
#include "avfilter.h"
#include "libavutil/intreadwrite.h"

enum MatrixMode {
    MATRIX_SQUARE,
    MATRIX_ROW,
    MATRIX_COLUMN,
    MATRIX_NBMODES,
};

typedef struct ConvolutionContext {
    const AVClass *class;

    char *matrix_str[4];
    float user_rdiv[4];
    float bias[4];
    int mode[4];
    float scale;
    float delta;
    int planes;

    float rdiv[4];
    int size[4];
    int depth;
    int max;
    int bpc;
    int nb_planes;
    int nb_threads;
    int planewidth[4];
    int planeheight[4];
    int matrix[4][49];
    int matrix_length[4];
    int copy[4];

    void (*setup[4])(int radius, const uint8_t *c[], const uint8_t *src, int stride,
                     int x, int width, int y, int height, int bpc);
    void (*filter[4])(uint8_t *dst, int width,
                      float rdiv, float bias, const int *const matrix,
                      const uint8_t *c[], int peak, int radius,
                      int dstride, int stride, int size);
} ConvolutionContext;

void ff_convolution_init_x86(ConvolutionContext *s);
void ff_sobel_init_x86(ConvolutionContext *s, int depth, int nb_planes);

static void setup_3x3(int radius, const uint8_t *c[], const uint8_t *src, int stride,
                      int x, int w, int y, int h, int bpc)
{
    int i;

    for (i = 0; i < 9; i++) {
        int xoff = FFABS(x + ((i % 3) - 1));
        int yoff = FFABS(y + (i / 3) - 1);

        xoff = xoff >= w ? 2 * w - 1 - xoff : xoff;
        yoff = yoff >= h ? 2 * h - 1 - yoff : yoff;

        c[i] = src + xoff * bpc + yoff * stride;
    }
}

static void filter_sobel(uint8_t *dst, int width,
                         float scale, float delta, const int *const matrix,
                         const uint8_t *c[], int peak, int radius,
                         int dstride, int stride, int size)
{
    const uint8_t *c0 = c[0], *c1 = c[1], *c2 = c[2];
    const uint8_t *c3 = c[3], *c5 = c[5];
    const uint8_t *c6 = c[6], *c7 = c[7], *c8 = c[8];
    int x;

    for (x = 0; x < width; x++) {
        float suma = c0[x] * -1 + c1[x] * -2 + c2[x] * -1 +
                     c6[x] *  1 + c7[x] *  2 + c8[x] *  1;
        float sumb = c0[x] * -1 + c2[x] *  1 + c3[x] * -2 +
                     c5[x] *  2 + c6[x] * -1 + c8[x] *  1;

        dst[x] = av_clip_uint8(sqrtf(suma*suma + sumb*sumb) * scale + delta);
    }
}

static void filter16_sobel(uint8_t *dstp, int width,
                           float scale, float delta, const int *const matrix,
                           const uint8_t *c[], int peak, int radius,
                           int dstride, int stride, int size)
{
    uint16_t *dst = (uint16_t *)dstp;
    int x;

    for (x = 0; x < width; x++) {
        float suma = AV_RN16A(&c[0][2 * x]) * -1 + AV_RN16A(&c[1][2 * x]) * -2 + AV_RN16A(&c[2][2 * x]) * -1 +
                     AV_RN16A(&c[6][2 * x]) *  1 + AV_RN16A(&c[7][2 * x]) *  2 + AV_RN16A(&c[8][2 * x]) *  1;
        float sumb = AV_RN16A(&c[0][2 * x]) * -1 + AV_RN16A(&c[2][2 * x]) *  1 + AV_RN16A(&c[3][2 * x]) * -2 +
                     AV_RN16A(&c[5][2 * x]) *  2 + AV_RN16A(&c[6][2 * x]) * -1 + AV_RN16A(&c[8][2 * x]) *  1;

        dst[x] = av_clip(sqrtf(suma*suma + sumb*sumb) * scale + delta, 0, peak);
    }
}

static inline void ff_sobel_init(ConvolutionContext *s, int depth, int nb_planes)
{
    for (int i = 0; i < 4; i++) {
        s->filter[i] = filter_sobel;
        s->copy[i] = !((1 << i) & s->planes);
        s->size[i] = 3;
        s->setup[i] = setup_3x3;
        s->rdiv[i] = s->scale;
        s->bias[i] = s->delta;
    }
    if (s->depth > 8)
        for (int i = 0; i < 4; i++)
            s->filter[i] = filter16_sobel;
#if ARCH_X86_64
    ff_sobel_init_x86(s, depth, nb_planes);
#endif
}
#endif
