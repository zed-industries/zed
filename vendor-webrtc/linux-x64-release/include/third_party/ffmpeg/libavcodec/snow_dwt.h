/*
 * Copyright (C) 2004-2010 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_SNOW_DWT_H
#define AVCODEC_SNOW_DWT_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/attributes.h"

struct MpegEncContext;

typedef int DWTELEM;
typedef short IDWTELEM;

#define MAX_DECOMPOSITIONS 8

typedef struct DWTCompose {
    IDWTELEM *b0;
    IDWTELEM *b1;
    IDWTELEM *b2;
    IDWTELEM *b3;
    int y;
} DWTCompose;

/** Used to minimize the amount of memory used in order to
 *  optimize cache performance. **/
typedef struct slice_buffer_s {
    IDWTELEM **line;   ///< For use by idwt and predict_slices.
    IDWTELEM **data_stack;   ///< Used for internal purposes.
    int data_stack_top;
    int line_count;
    int line_width;
    int data_count;
    IDWTELEM *base_buffer;  ///< Buffer that this structure is caching.
} slice_buffer;

struct SnowDWTContext;

typedef struct SnowDWTContext {
    void (*vertical_compose97i)(IDWTELEM *b0, IDWTELEM *b1, IDWTELEM *b2,
                                IDWTELEM *b3, IDWTELEM *b4, IDWTELEM *b5,
                                int width);
    void (*horizontal_compose97i)(IDWTELEM *b, IDWTELEM *temp, int width);
    void (*inner_add_yblock)(const uint8_t *obmc, const int obmc_stride,
                             uint8_t **block, int b_w, int b_h, int src_x,
                             int src_y, int src_stride, slice_buffer *sb,
                             int add, uint8_t *dst8);
} SnowDWTContext;


#define DWT_97 0
#define DWT_53 1

#define liftS lift
#define W_AM 3
#define W_AO 0
#define W_AS 1

#undef liftS
#define W_BM 1
#define W_BO 8
#define W_BS 4

#define W_CM 1
#define W_CO 0
#define W_CS 0

#define W_DM 3
#define W_DO 4
#define W_DS 3

#define slice_buffer_get_line(slice_buf, line_num)                          \
    ((slice_buf)->line[line_num] ? (slice_buf)->line[line_num]              \
                                 : ff_slice_buffer_load_line((slice_buf),   \
                                                             (line_num)))

/* C bits used by mmx/sse2/altivec */

static av_always_inline void snow_interleave_line_header(int *i, int width, IDWTELEM *low, IDWTELEM *high)
{
    *i = width - 2;

    if (width & 1) {
        low[*i + 1] = low[(*i + 1)>>1];
        (*i)--;
    }
}

static av_always_inline void snow_interleave_line_footer(int *i, IDWTELEM *low, const IDWTELEM *high)
{
    for (; *i >= 0; *i -= 2) {
        low[*i + 1] = high[*i >> 1];
        low[*i]     =  low[*i >> 1];
    }
}

static av_always_inline void snow_horizontal_compose_lift_lead_out(int i, IDWTELEM *dst, const IDWTELEM *src, const IDWTELEM *ref, int width, int w, int lift_high, int mul, int add, int shift)
{
    for (; i < w; i++)
        dst[i] = src[i] - ((mul * (ref[i] + ref[i + 1]) + add) >> shift);

    if ((width ^ lift_high) & 1)
        dst[w] = src[w] - ((mul * 2 * ref[w] + add) >> shift);
}

static av_always_inline void snow_horizontal_compose_liftS_lead_out(int i, IDWTELEM *dst, const IDWTELEM *src, const IDWTELEM *ref, int width, int w)
{
    for (; i < w; i++)
        dst[i] = src[i] + ((ref[i] + ref[(i+1)]+W_BO + 4 * src[i]) >> W_BS);

    if (width & 1)
        dst[w] = src[w] + ((2 * ref[w] + W_BO + 4 * src[w]) >> W_BS);
}

int ff_slice_buffer_init(slice_buffer *buf, int line_count,
                         int max_allocated_lines, int line_width,
                         IDWTELEM *base_buffer);
void ff_slice_buffer_release(slice_buffer *buf, int line);
void ff_slice_buffer_flush(slice_buffer *buf);
void ff_slice_buffer_destroy(slice_buffer *buf);
IDWTELEM *ff_slice_buffer_load_line(slice_buffer *buf, int line);

void ff_snow_inner_add_yblock(const uint8_t *obmc, const int obmc_stride,
                              uint8_t **block, int b_w, int b_h, int src_x,
                              int src_y, int src_stride, slice_buffer *sb,
                              int add, uint8_t *dst8);

int ff_w53_32_c(struct MpegEncContext *v, const uint8_t *pix1, const uint8_t *pix2, ptrdiff_t line_size, int h);
int ff_w97_32_c(struct MpegEncContext *v, const uint8_t *pix1, const uint8_t *pix2, ptrdiff_t line_size, int h);

void ff_spatial_dwt(int *buffer, int *temp, int width, int height, int stride,
                    int type, int decomposition_count);

void ff_spatial_idwt_buffered_init(DWTCompose *cs, slice_buffer *sb, int width,
                                   int height, int stride_line, int type,
                                   int decomposition_count);
void ff_spatial_idwt_buffered_slice(SnowDWTContext *dsp, DWTCompose *cs,
                                    slice_buffer *slice_buf, IDWTELEM *temp,
                                    int width, int height, int stride_line,
                                    int type, int decomposition_count, int y);
void ff_spatial_idwt(IDWTELEM *buffer, IDWTELEM *temp, int width, int height,
                     int stride, int type, int decomposition_count);

void ff_dwt_init(SnowDWTContext *c);
void ff_dwt_init_x86(SnowDWTContext *c);

#endif /* AVCODEC_DWT_H */
